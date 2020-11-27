// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Tectonic document definitions.

use std::{
    env, fs,
    io::{Read, Write},
    path::{Component, PathBuf},
};

use crate::{
    config, ctry, driver::ProcessingSessionBuilder, errors::Result,
    io::cached_itarbundle::resolve_url, status::StatusBackend, workspace::WorkspaceCreator,
};

/// A Tectonic document.
#[derive(Debug)]
pub struct Document {
    /// The directory containing the `Tectonic.toml` file and document source.
    src_dir: PathBuf,

    /// The directory where document build artifacts will be output. By default
    /// this will be a subdirectory of `src_dir` named `build`.
    build_dir: PathBuf,

    /// The document name. This will be used to name build artifacts and the
    /// like, and so should be relatively filesystem-friendly. It does not
    /// need to be the same as the document title.
    name: String,

    /// The name of core TeX file bundle upon which this document is based.
    /// Either a URL or a local path.
    bundle_loc: String,
}

impl Document {
    /// Initialize a Document based on a TOML specification
    pub(crate) fn new_from_toml<R: Read>(
        src_dir: PathBuf,
        build_dir: PathBuf,
        toml_data: &mut R,
    ) -> Result<Self> {
        let mut toml_text = String::new();
        toml_data.read_to_string(&mut toml_text)?;
        let doc: syntax::Document = toml::from_str(&toml_text)?;

        Ok(Document {
            src_dir,
            build_dir,
            name: doc.doc.name,
            bundle_loc: doc.doc.bundle,
        })
    }

    /// Create a new in-memory Document, based on the settings of a
    /// WorkspaceCreator object.
    pub(crate) fn new_for_creator(
        wc: &WorkspaceCreator,
        config: &config::PersistentConfig,
        status: &mut dyn StatusBackend,
    ) -> Result<Self> {
        let src_dir = wc.root_dir().to_owned();

        let mut build_dir = src_dir.clone();
        build_dir.push("build");

        // We're a bit roundabout in how we figure out the name of the
        // containing src_dir, in an effort to Do The Right Thing with symlinks
        // and whatnot.
        let name = {
            let mut name = "document".to_owned();
            let mut tried_src_path = false;

            if let Some(c) = src_dir.components().next_back() {
                if let Component::Normal(t) = c {
                    tried_src_path = true;

                    if let Some(s) = t.to_str() {
                        name = s.to_owned();
                    }
                }
            }

            if !tried_src_path {
                if let Ok(cwd) = env::current_dir() {
                    let full_path = cwd.join(&src_dir);

                    if let Some(c) = full_path.components().next_back() {
                        if let Component::Normal(t) = c {
                            if let Some(s) = t.to_str() {
                                name = s.to_owned();
                            }
                        }
                    }
                }
            }

            name
        };

        // Determine the bundle URL that we'll put in as the default.

        let bundle_loc = resolve_url(config.default_bundle_loc(), status)?;

        // All done.
        Ok(Document {
            src_dir,
            build_dir,
            name,
            bundle_loc,
        })
    }

    /// Write out this document's state as a new TOML file. This should only be
    /// used when creating a totally new document; otherwise TOML rewriting
    /// should be used.
    pub(crate) fn create_toml(&self) -> Result<()> {
        let doc = syntax::Document {
            doc: syntax::DocSection {
                name: self.name.clone(),
                bundle: self.bundle_loc.clone(),
            },
        };

        let toml_text = toml::to_string_pretty(&doc)?;

        let mut toml_path = self.src_dir.clone();
        toml_path.push("Tectonic.toml");

        let mut toml_file = ctry!(fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&toml_path);
            "couldn\'t create `{}`", toml_path.display()
        );

        toml_file.write_all(toml_text.as_bytes())?;
        Ok(())
    }
}

/// The concrete syntax for saving document state, wired up via serde.
mod syntax {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize)]
    pub struct Document {
        pub doc: DocSection,
    }

    #[derive(Debug, Deserialize, Serialize)]
    pub struct DocSection {
        pub name: String,
        pub bundle: String,
    }
}
