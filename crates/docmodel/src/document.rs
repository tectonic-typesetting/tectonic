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

use crate::syntax;
use crate::workspace::WorkspaceCreator;

/// The default files used to build a document.
///
/// This default can be overridden on an output-by-output basis in
/// `Tectonic.toml`.
pub const DEFAULT_INPUTS: &[&str] = &["_preamble.tex", "index.tex", "_postamble.tex"];

/// A Tectonic document.
#[derive(Debug)]
pub struct Document {
    /// The directory containing the `Tectonic.toml` file and document source.
    src_dir: PathBuf,

    /// The directory where document build artifacts will be output. By default
    /// this will be a subdirectory of `src_dir` named `build`.
    build_dir: PathBuf,

    /// Arbitrary document metadata.
    /// This has no effect on tectonic's build process.
    /// Rather, allows users to add easily-accessible information to their documents,
    /// which may be read by external tools.
    pub metadata: Option<toml::Value>,

    /// The document name. This will be used to name build artifacts and the
    /// like, and so should be relatively filesystem-friendly. It does not
    /// need to be the same as the document title.
    pub name: String,

    /// The name of core TeX file bundle upon which this document is based.
    /// Either a URL or a local path.
    pub bundle_loc: String,

    /// Extra local search paths for this document.
    /// May be absolute or relative to src_dir.
    pub extra_paths: Vec<PathBuf>,

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
        let doc: syntax::TomlDocument = toml::from_str(&toml_text)?;

        let mut outputs = HashMap::new();

        for toml_output in &doc.outputs {
            let output: OutputProfile = toml_output.into();

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
            extra_paths: doc.doc.extra_paths.unwrap_or_default(),
            metadata: doc.doc.metadata,
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
            .map(syntax::TomlOutputProfile::from)
            .collect();

        let extra_paths = if self.extra_paths.is_empty() {
            None
        } else {
            Some(self.extra_paths.clone())
        };

        let doc = syntax::TomlDocument {
            doc: syntax::TomlDocSection {
                name: self.name.clone(),
                bundle: self.bundle_loc.clone(),
                extra_paths,
                metadata: None,
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

    /// The input files we should use to build this document
    pub inputs: Vec<InputFile>,

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

/// An input provided to a document build
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InputFile {
    /// A file system path.
    File(String),

    /// An inline file.
    Inline(String),
}

impl Document {
    /// Create a new in-memory Document, based on the settings of a
    /// WorkspaceCreator object.
    pub(crate) fn create_for(
        wc: &WorkspaceCreator,
        bundle_loc: String,
        extra_paths: Vec<PathBuf>,
    ) -> Result<Self> {
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
            extra_paths,
            outputs: crate::document::default_outputs(),
            metadata: None,
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
            inputs: DEFAULT_INPUTS
                .iter()
                .map(|x| InputFile::File(x.to_string()))
                .collect(),
            shell_escape: false,
            shell_escape_cwd: None,
        },
    );
    outputs
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    #[test]
    fn shell_escape_default_false() {
        const TOML: &str = r#"
        [doc]
        name = "test"
        bundle = "na"

        [[output]]
        name = "o"
        type = "pdf"
        "#;

        let mut c = Cursor::new(TOML.as_bytes());
        let doc = Document::new_from_toml(".", ".", &mut c).unwrap();
        assert!(!doc.outputs.get("o").unwrap().shell_escape);
    }

    #[test]
    fn shell_escape_cwd_implies_shell_escape() {
        const TOML: &str = r#"
        [doc]
        name = "test"
        bundle = "na"

        [[output]]
        name = "o"
        type = "pdf"
        shell_escape_cwd = "."
        "#;

        let mut c = Cursor::new(TOML.as_bytes());
        let doc = Document::new_from_toml(".", ".", &mut c).unwrap();
        assert!(doc.outputs.get("o").unwrap().shell_escape);
    }
}
