// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! A single Tectonic document.
//!
//! Every document is part of a [`crate::workspace::Workspace`]. At the moment
//! workspaces can only contain a single document each, but in the future it
//! might become possible for one workspace to contain multiple documents.
//!
//! This crate, on its own, does not provide document-processing capabilities.
//! The main `tectonic` crate provides extension traits that set up document
//! processing, in the `tectonic::docmodel` module.

use std::{
    collections::HashMap,
    env, fs,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};
use tectonic_errors::prelude::*;

use crate::workspace::WorkspaceCreator;

/// The default filesystem name for the "preamble" file of a document.
///
/// This default can be overridden on an output-by-output basis in
/// `Tectonic.toml`.
pub const DEFAULT_PREAMBLE_FILE: &str = "_preamble.tex";

/// The default filesystem name for the main "index" file of a document.
///
/// This default can be overridden on an output-by-output basis in
/// `Tectonic.toml`.
pub const DEFAULT_INDEX_FILE: &str = "index.tex";

/// The default filesystem name for the "postamble" file of a document.
///
/// This default can be overridden on an output-by-output basis in
/// `Tectonic.toml`.
pub const DEFAULT_POSTAMBLE_FILE: &str = "_postamble.tex";

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
    pub name: String,

    /// The name of core TeX file bundle upon which this document is based.
    /// Either a URL or a local path.
    pub bundle_loc: String,

    /// The different outputs that are created from the document source. These
    /// may have different formats (e.g., PDF and HTML) or the same format but
    /// different settings (e.g., PDF with A4 paper and PDF with US Letter
    /// paper).
    pub outputs: HashMap<String, OutputProfile>,
}

impl Document {
    /// Initialize a Document based on a TOML specification.
    ///
    /// This function can initialize a document directly from a TOML-formatted
    /// data stream. In many circumstances you shouldn’t use it; instead you
    /// should open a [`crate::workspace::Workspace`] and get a [`Document`]
    /// through it.
    pub fn new_from_toml<P1: Into<PathBuf>, P2: Into<PathBuf>, R: Read>(
        src_dir: P1,
        build_dir: P2,
        toml_data: &mut R,
    ) -> Result<Self> {
        let mut toml_text = String::new();
        toml_data.read_to_string(&mut toml_text)?;
        let doc: syntax::Document = toml::from_str(&toml_text)?;

        let mut outputs = HashMap::new();

        for toml_output in &doc.outputs {
            let output = toml_output.to_runtime();

            if outputs.insert(output.name.clone(), output).is_some() {
                bail!(
                    "duplicated output name `{}` in TOML specification",
                    &toml_output.name
                );
            }
        }

        if outputs.is_empty() {
            bail!("TOML specification must define at least one output");
        }

        Ok(Document {
            src_dir: src_dir.into(),
            build_dir: build_dir.into(),
            name: doc.doc.name,
            bundle_loc: doc.doc.bundle,
            outputs,
        })
    }

    /// Write out this document's state as a fresh `Tectonic.toml` file in the
    /// document’s [`Self::src_dir`].
    ///
    /// This should only be used when creating a totally new document. Otherwise
    /// TOML rewriting should be used, to preserve the user's file structure,
    /// comments, etc.
    pub fn create_toml(&self) -> Result<()> {
        let outputs = self
            .outputs
            .values()
            .map(syntax::OutputProfile::from_runtime)
            .collect();

        let doc = syntax::Document {
            doc: syntax::DocSection {
                name: self.name.clone(),
                bundle: self.bundle_loc.clone(),
            },
            outputs,
        };

        let toml_text = toml::to_string_pretty(&doc)?;

        let mut toml_path = self.src_dir.clone();
        toml_path.push("Tectonic.toml");

        let mut toml_file = atry!(fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&toml_path);
            ["couldn\'t create `{}`", toml_path.display()]
        );

        toml_file.write_all(toml_text.as_bytes())?;
        Ok(())
    }

    /// Get this document's toplevel source directory.
    ///
    /// Note that this directory is the one containing the file `Tectonic.toml`.
    /// The actual document source is contained in a subdirectory named `src`.
    pub fn src_dir(&self) -> &Path {
        &self.src_dir
    }

    /// Get this document's build directory.
    ///
    /// This is the directory where persistent files associated with the
    /// document build are stored. By default, it is a subdirectory of
    /// [`Self::src_dir`] named `build`.
    pub fn build_dir(&self) -> &Path {
        &self.build_dir
    }

    /// Iterate over the names of the output profiles defined for this document.
    ///
    /// These may have different formats (e.g., PDF and HTML) or the same format
    /// but different settings (e.g., PDF with A4 paper and PDF with US Letter
    /// paper).
    pub fn output_names(&self) -> impl Iterator<Item = &str> {
        self.outputs.keys().map(|k| k.as_ref())
    }

    /// Get the path of the "main" output file for the given output profile.
    ///
    /// The exact meaning of "main" will depend on the output format.
    pub fn output_main_file(&self, profile_name: &str) -> PathBuf {
        let profile = self.outputs.get(profile_name).unwrap();

        let mut p = self.build_dir.clone();
        p.push(&profile.name);

        match profile.target_type {
            BuildTargetType::Pdf => {
                p.push(&profile.name);
                p.set_extension("pdf");
            }

            BuildTargetType::Html => {
                p.push("index.html");
            }
        }

        p
    }
}

/// Persistent settings for a document build.
#[derive(Clone, Debug)]
pub struct OutputProfile {
    /// The name of this profile.
    pub name: String,

    /// The type of output targeted by this profile.
    pub target_type: BuildTargetType,

    /// The name of the TeX format used by this profile.
    pub tex_format: String,

    /// The name of the preamble file within the `src` directory.
    pub preamble_file: String,

    /// The name of the index (main) file within the `src` directory.
    pub index_file: String,

    /// The name of the postamble file within the `src` directory.
    pub postamble_file: String,

    /// Whether TeX's shell-escape feature should be activated in this profile.
    ///
    /// Note that besides creating portability and reproducibility issues,
    /// shell-escape opens enormous security holes. It should only ever be
    /// activated with fully trusted input.
    pub shell_escape: bool,

    /// Directory to use as the cwd for shell escaped execution.
    ///
    /// Setting this to $(pwd) gives the same relative path shell-escape behaviour
    /// (e.g. for \inputminted), as other engines, such as xelatex
    ///
    /// Directory is not managed and any files created in it will not be deleted.
    ///
    pub shell_escape_cwd: Option<String>,
}

/// The output target type of a document build.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BuildTargetType {
    /// Output a tree of HTML files
    Html,

    /// Output to the Portable Document Format (PDF).
    Pdf,
}

impl Document {
    /// Create a new in-memory Document, based on the settings of a
    /// WorkspaceCreator object.
    pub(crate) fn create_for(wc: &WorkspaceCreator, bundle_loc: String) -> Result<Self> {
        let src_dir = wc.root_dir.clone();

        let mut build_dir = src_dir.clone();
        build_dir.push("build");

        // We're a bit roundabout in how we figure out the name of the
        // containing src_dir, in an effort to Do The Right Thing with symlinks
        // and whatnot.
        let name = {
            let mut name = "document".to_owned();
            let mut tried_src_path = false;

            if let Some(Component::Normal(t)) = src_dir.components().next_back() {
                tried_src_path = true;

                if let Some(s) = t.to_str() {
                    name = s.to_owned();
                }
            }

            if !tried_src_path {
                if let Ok(cwd) = env::current_dir() {
                    let full_path = cwd.join(&src_dir);

                    if let Some(Component::Normal(t)) = full_path.components().next_back() {
                        if let Some(s) = t.to_str() {
                            name = s.to_owned();
                        }
                    }
                }
            }

            name
        };

        // All done.
        Ok(Document {
            src_dir,
            build_dir,
            name,
            bundle_loc,
            outputs: crate::document::default_outputs(),
        })
    }
}

pub(crate) fn default_outputs() -> HashMap<String, OutputProfile> {
    let mut outputs = HashMap::new();
    outputs.insert(
        "default".to_owned(),
        OutputProfile {
            name: "default".to_owned(),
            target_type: BuildTargetType::Pdf,
            tex_format: "latex".to_owned(),
            preamble_file: DEFAULT_PREAMBLE_FILE.to_owned(),
            index_file: DEFAULT_INDEX_FILE.to_owned(),
            postamble_file: DEFAULT_POSTAMBLE_FILE.to_owned(),
            shell_escape: false,
            shell_escape_cwd: None,
        },
    );
    outputs
}

/// The concrete syntax for saving document state, wired up via serde.
mod syntax {
    use super::{DEFAULT_INDEX_FILE, DEFAULT_POSTAMBLE_FILE, DEFAULT_PREAMBLE_FILE};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct Document {
        pub doc: DocSection,

        #[serde(rename = "output")]
        pub outputs: Vec<OutputProfile>,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct DocSection {
        pub name: String,
        pub bundle: String,
    }

    #[derive(Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct OutputProfile {
        pub name: String,
        #[serde(rename = "type")]
        pub target_type: BuildTargetType,
        pub tex_format: Option<String>,
        #[serde(rename = "preamble")]
        pub preamble_file: Option<String>,
        #[serde(rename = "index")]
        pub index_file: Option<String>,
        #[serde(rename = "postamble")]
        pub postamble_file: Option<String>,
        pub shell_escape: Option<bool>,
        pub shell_escape_cwd: Option<String>,
    }

    impl OutputProfile {
        pub fn from_runtime(rt: &super::OutputProfile) -> Self {
            let tex_format = if rt.tex_format == "latex" {
                None
            } else {
                Some(rt.tex_format.clone())
            };

            let preamble_file = if rt.preamble_file == DEFAULT_PREAMBLE_FILE {
                None
            } else {
                Some(rt.preamble_file.clone())
            };

            let index_file = if rt.index_file == DEFAULT_INDEX_FILE {
                None
            } else {
                Some(rt.index_file.clone())
            };

            let postamble_file = if rt.postamble_file == DEFAULT_POSTAMBLE_FILE {
                None
            } else {
                Some(rt.postamble_file.clone())
            };

            let shell_escape = if !rt.shell_escape { None } else { Some(true) };
            let shell_escape_cwd = rt.shell_escape_cwd.clone();

            OutputProfile {
                name: rt.name.clone(),
                target_type: BuildTargetType::from_runtime(&rt.target_type),
                tex_format,
                preamble_file,
                index_file,
                postamble_file,
                shell_escape,
                shell_escape_cwd,
            }
        }

        pub fn to_runtime(&self) -> super::OutputProfile {
            super::OutputProfile {
                name: self.name.clone(),
                target_type: self.target_type.to_runtime(),
                tex_format: self
                    .tex_format
                    .as_ref()
                    .map(|s| s.as_ref())
                    .unwrap_or("latex")
                    .to_owned(),
                preamble_file: self
                    .preamble_file
                    .clone()
                    .unwrap_or_else(|| DEFAULT_PREAMBLE_FILE.to_owned()),
                index_file: self
                    .index_file
                    .clone()
                    .unwrap_or_else(|| DEFAULT_INDEX_FILE.to_owned()),
                postamble_file: self
                    .postamble_file
                    .clone()
                    .unwrap_or_else(|| DEFAULT_POSTAMBLE_FILE.to_owned()),
                shell_escape: self.shell_escape.unwrap_or_default(),
                shell_escape_cwd: self.shell_escape_cwd.clone(),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum BuildTargetType {
        Html,
        Pdf,
    }

    impl BuildTargetType {
        pub fn from_runtime(rt: &super::BuildTargetType) -> Self {
            match rt {
                super::BuildTargetType::Html => BuildTargetType::Html,
                super::BuildTargetType::Pdf => BuildTargetType::Pdf,
            }
        }

        pub fn to_runtime(self) -> super::BuildTargetType {
            match self {
                BuildTargetType::Html => super::BuildTargetType::Html,
                BuildTargetType::Pdf => super::BuildTargetType::Pdf,
            }
        }
    }

    impl Serialize for BuildTargetType {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(match *self {
                BuildTargetType::Html => "html",
                BuildTargetType::Pdf => "pdf",
            })
        }
    }
    impl<'de> Deserialize<'de> for BuildTargetType {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Ok(match s.as_str() {
                "html" => BuildTargetType::Html,
                "pdf" => BuildTargetType::Pdf,
                other => {
                    return Err(<D as Deserializer>::Error::unknown_variant(
                        other,
                        &["html", "pdf"],
                    ))
                }
            })
        }
    }
}
