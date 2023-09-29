// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! A Tectonic document-build workspace.
//!
//! For the time being, this is just a thin wrapper to provide access to a
//! `Document` instance. This API exists to future-proof a bit for a potential
//! world where one workspace can contain multiple documents.

use std::{
    env,
    error::Error,
    fmt, fs,
    io::{self, Write},
    path::PathBuf,
};
use tectonic_errors::prelude::*;

use crate::document::Document;

/// A Tectonic workspace.
///
/// For the time being, a Workspace is just a thin wrapper to provide access to
/// a `Document` instance. In the future, it might become possible for one
/// workspace to contain multiple documents.
///
/// In most cases, you will want to create a [`Workspace`] by opening an
/// existing one using [`Workspace::open_from_environment`].
#[derive(Debug)]
pub struct Workspace {
    /// The root directory of the workspace.
    #[allow(dead_code)] // We expect to use this eventually.
    root_dir: PathBuf,

    /// This workspace's document. In the future, there might be more than one.
    doc: Document,
}

impl Workspace {
    /// Get the first document in the workspace.
    ///
    /// Right now, workspaces in fact only include one document. That may change
    /// in the future.
    pub fn first_document(&self) -> &Document {
        &self.doc
    }

    /// Get the first document in the workspace, mutably.
    ///
    /// Right now, workspaces in fact only include one document. That may change
    /// in the future.
    pub fn first_document_mut(&mut self) -> &mut Document {
        &mut self.doc
    }

    /// Open up a workspace based on the current process environment.
    ///
    /// This function searches the current directory and its parents for a
    /// `Tectonic.toml` file. Because workspaces can currently only contain a
    /// single document, the search stops when the first such file is found. If
    /// no such file is found, an error downcastable into
    /// [`NoWorkspaceFoundError`] is returned.
    pub fn open_from_environment() -> Result<Self> {
        let initial_dir = env::current_dir()?;

        let mut root_dir = initial_dir.clone();
        root_dir.push("tmp"); // simplifies loop logic

        static TOML_PATHS: &[&str] = &["Tectonic.toml", "tectonic.toml"];

        let mut doc_file: Option<fs::File> = None;
        while root_dir.pop() {
            // Try all known config paths
            for path in TOML_PATHS {
                root_dir.push(path);

                match fs::File::open(&root_dir) {
                    Ok(f) => {
                        // We found a file, use it.
                        doc_file = Some(f);
                        break;
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                        root_dir.pop(); // remove this path, try next file in array
                    }
                    Err(e) => return Err(e.into()),
                };
            }

            // If we didn't find a config file in this dir, try the parent.
            if doc_file.is_none() {
                continue;
            }

            let mut doc_file = doc_file.unwrap();
            root_dir.pop();
            let mut doc_build_dir = root_dir.clone();
            doc_build_dir.push("build");
            let doc = Document::new_from_toml(root_dir.clone(), doc_build_dir, &mut doc_file)?;

            return Ok(Workspace { root_dir, doc });
        }
        Err(NoWorkspaceFoundError { initial_dir }.into())
    }
}

/// An error for when the environment does not seem to contain a Tectonic
/// workspace.
#[derive(Debug)]
pub struct NoWorkspaceFoundError {
    initial_dir: PathBuf,
}

impl fmt::Display for NoWorkspaceFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> StdResult<(), fmt::Error> {
        write!(
            f,
            "could not find `Tectonic.toml` in `{}` or any parent directory",
            self.initial_dir.display()
        )
    }
}

impl Error for NoWorkspaceFoundError {}

/// A type for creating a new workspace.
#[derive(Debug)]
pub struct WorkspaceCreator {
    /// The root directory of the workspace to be created.
    pub(crate) root_dir: PathBuf,
}

impl WorkspaceCreator {
    /// Initialize a `WorkspaceCreator` variable.
    pub fn new<P: Into<PathBuf>>(root_dir: P) -> Self {
        WorkspaceCreator {
            root_dir: root_dir.into(),
        }
    }

    /// Consume this object and attempt to create the new workspace.
    pub fn create(self, bundle_loc: String) -> Result<Workspace> {
        let doc = Document::create_for(&self, bundle_loc)?;

        let mut tex_dir = self.root_dir.clone();
        tex_dir.push("src");

        atry!(
            fs::create_dir_all(&tex_dir);
            ["couldn\'t create workspace directory `{}`", tex_dir.display()]
        );

        doc.create_toml()?;

        // Stub out the TeX.

        {
            tex_dir.push("_preamble.tex");
            let mut f = fs::File::create(&tex_dir)?;
            f.write_all(
                br"\documentclass{article}
\title{My Title}
\begin{document}
",
            )?;
            tex_dir.pop();
        }

        {
            tex_dir.push("index.tex");
            let mut f = fs::File::create(&tex_dir)?;
            f.write_all(
                br"Hello, world.
",
            )?;
            tex_dir.pop();
        }

        {
            tex_dir.push("_postamble.tex");
            let mut f = fs::File::create(&tex_dir)?;
            f.write_all(
                br"\end{document}
",
            )?;
            tex_dir.pop();
        }

        // All done.

        Ok(Workspace {
            root_dir: self.root_dir,
            doc,
        })
    }
}
