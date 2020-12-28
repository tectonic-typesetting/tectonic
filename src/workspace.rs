// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! A Tectonic document-build workspace.
//!
//! For the time being, this is just a thin wrapper to provide access to a
//! `Document` instance. This API exists to future-proof a bit for a potential
//! world where one workspace can contain multiple documents.

use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use crate::{
    config::PersistentConfig, ctry, document::Document, errmsg, errors::Result,
    status::StatusBackend,
};

/// A Tectonic workspace.
#[derive(Debug)]
pub struct Workspace {
    /// The root directory of the workspace.
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

    /// Open up a workspace baced on the current process environment.
    pub fn open_from_environment() -> Result<Self> {
        let mut root_dir = env::current_dir()?;
        root_dir.push("tmp"); // simplifies loop logic

        while root_dir.pop() {
            root_dir.push("Tectonic.toml");

            let mut doc_file = match fs::File::open(&root_dir) {
                Ok(f) => f,
                Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
                    root_dir.pop(); // remove "Tectonic.toml"
                    continue; // this will pop up one directory and try again
                }
                Err(e) => return Err(e.into()),
            };

            root_dir.pop();
            let mut doc_build_dir = root_dir.clone();
            doc_build_dir.push("build");
            let doc = Document::new_from_toml(root_dir.clone(), doc_build_dir, &mut doc_file)?;

            return Ok(Workspace { root_dir, doc });
        }

        Err(errmsg!(
            "No `Tectonic.toml` found in current directory or any of its parents"
        ))
    }
}

/// A type for creating a new workspace.
#[derive(Debug)]
pub struct WorkspaceCreator {
    /// The root directory of the workspace to be created.
    root_dir: PathBuf,
}

impl WorkspaceCreator {
    /// Initialize a `WorkspaceCreator` variable.
    pub fn new<P: Into<PathBuf>>(root_dir: P) -> Self {
        WorkspaceCreator {
            root_dir: root_dir.into(),
        }
    }

    /// Consume this object and attempt to create the new workspace.
    pub fn create(
        self,
        config: &PersistentConfig,
        status: &mut dyn StatusBackend,
    ) -> Result<Workspace> {
        let doc = Document::new_for_creator(&self, config, status)?;

        let mut tex_dir = self.root_dir.clone();
        tex_dir.push("src");

        ctry!(
            fs::create_dir_all(&tex_dir);
            "couldn\'t create workspace directory `{}`", tex_dir.display()
        );

        doc.create_toml()?;

        // Stub out the TeX.

        {
            tex_dir.push("_preamble.tex");
            let mut f = fs::File::create(&tex_dir)?;
            f.write_all(
                br#"\documentclass{article}
\title{My Title}
\begin{document}
"#,
            )?;
            tex_dir.pop();
        }

        {
            tex_dir.push("index.tex");
            let mut f = fs::File::create(&tex_dir)?;
            f.write_all(
                br#"Hello, world.
"#,
            )?;
            tex_dir.pop();
        }

        {
            tex_dir.push("_postamble.tex");
            let mut f = fs::File::create(&tex_dir)?;
            f.write_all(
                br#"\end{document}
"#,
            )?;
            tex_dir.pop();
        }

        // All done.

        Ok(Workspace {
            root_dir: self.root_dir,
            doc,
        })
    }

    /// Get the root directory of the workspace.
    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}
