// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! Connecting the Tectonic document model to the engines.
//!
//! This module extends the document model types provided by the
//! `tectonic_docmodel` crate with the actual document-processing capabilities
//! provided by the processing engines.

use std::{
    fmt::Write as FmtWrite,
    fs, io,
    path::{Path, PathBuf},
};
use tectonic_bridge_core::SecuritySettings;
use tectonic_bundles::{
    cache::Cache, dir::DirBundle, itar::IndexedTarBackend, zip::ZipBundle, Bundle,
};
use tectonic_docmodel::{
    document::{BuildTargetType, Document, InputFile},
    workspace::{Workspace, WorkspaceCreator},
};
use tectonic_geturl::{DefaultBackend, GetUrlBackend};
use url::Url;

use crate::{
    config, ctry,
    driver::{OutputFormat, PassSetting, ProcessingSessionBuilder},
    errors::{ErrorKind, Result},
    status::StatusBackend,
    tt_note,
    unstable_opts::UnstableOptions,
};

/// Options for setting up [`Document`] instances with the driver
#[derive(Clone, Debug, Default)]
pub struct DocumentSetupOptions {
    /// Disable requests to the network, if the document’s bundle happens to be
    /// network-based.
    only_cached: bool,

    /// Security settings for engine features.
    security: SecuritySettings,

    /// Ensure a deterministic build environment.
    deterministic_mode: bool,
}

impl DocumentSetupOptions {
    /// Create a new set of document setup options with custom security
    /// settings.
    pub fn new_with_security(security: SecuritySettings) -> Self {
        DocumentSetupOptions {
            only_cached: false,
            deterministic_mode: false,
            security,
        }
    }

    /// Specify whether any requests to the network will be made for bundle
    /// resources.
    ///
    /// If the document’s backing bundle is not network-based, this setting will
    /// have no effect.
    pub fn only_cached(&mut self, s: bool) -> &mut Self {
        self.only_cached = s;
        self
    }

    /// Specify whether we want to ensure a deterministic build environment.
    pub fn deterministic_mode(&mut self, s: bool) -> &mut Self {
        self.deterministic_mode = s;
        self
    }
}

pub trait DocumentExt {
    /// Get the bundle used by this document.
    ///
    /// This parses [`Document::bundle_loc`] and turns it into the appropriate
    /// bundle backend.
    fn bundle(
        &self,
        setup_options: &DocumentSetupOptions,
        status: &mut dyn StatusBackend,
    ) -> Result<Box<dyn Bundle>>;

    /// Set up a [`ProcessingSessionBuilder`] for one of the outputs.
    ///
    /// The *output_profile* argument gives the name of the document’s output profile to
    /// use.
    fn setup_session(
        &self,
        output_profile: &str,
        setup_options: &DocumentSetupOptions,
        status: &mut dyn StatusBackend,
    ) -> Result<ProcessingSessionBuilder>;
}

impl DocumentExt for Document {
    fn bundle(
        &self,
        setup_options: &DocumentSetupOptions,
        status: &mut dyn StatusBackend,
    ) -> Result<Box<dyn Bundle>> {
        fn bundle_from_path(p: PathBuf) -> Result<Box<dyn Bundle>> {
            if p.is_dir() {
                Ok(Box::new(DirBundle::new(p)))
            } else {
                Ok(Box::new(ZipBundle::open(p)?))
            }
        }

        if let Ok(test_bundle) = config::maybe_return_test_bundle(None) {
            Ok(test_bundle)
        } else if let Ok(url) = Url::parse(&self.bundle_loc) {
            if url.scheme() != "file" {
                let mut cache = Cache::get_user_default()?;
                let bundle = cache.open::<IndexedTarBackend>(
                    &self.bundle_loc,
                    setup_options.only_cached,
                    status,
                )?;
                Ok(Box::new(bundle))
            } else {
                let file_path = url.to_file_path().map_err(|_| {
                    io::Error::new(io::ErrorKind::InvalidInput, "failed to parse local path")
                })?;
                bundle_from_path(file_path)
            }
        } else {
            bundle_from_path(Path::new(&self.bundle_loc).to_owned())
        }
    }

    fn setup_session(
        &self,
        output_profile: &str,
        setup_options: &DocumentSetupOptions,
        status: &mut dyn StatusBackend,
    ) -> Result<ProcessingSessionBuilder> {
        let profile = self.outputs.get(output_profile).ok_or_else(|| {
            ErrorKind::Msg(format!(
                "unrecognized output profile name \"{output_profile}\""
            ))
        })?;

        let output_format = match profile.target_type {
            BuildTargetType::Html => OutputFormat::Html,
            BuildTargetType::Pdf => OutputFormat::Pdf,
        };

        let mut input_buffer = String::new();

        for input in &profile.inputs {
            match input {
                InputFile::Inline(s) => {
                    writeln!(input_buffer, "{}", s)?;
                }
                InputFile::File(f) => {
                    writeln!(input_buffer, "\\input{{{}}}", f)?;
                }
            };
        }

        let mut sess_builder =
            ProcessingSessionBuilder::new_with_security(setup_options.security.clone());

        // Interpret all extra paths as relative to our working dir
        let extra_paths: Vec<PathBuf> = self
            .extra_paths
            .iter()
            .map(|x| self.src_dir().join(x))
            .collect();

        sess_builder
            .output_format(output_format)
            .format_name(&profile.tex_format)
            .build_date_from_env(setup_options.deterministic_mode)
            .unstables(UnstableOptions {
                deterministic_mode: setup_options.deterministic_mode,
                extra_search_paths: extra_paths,
                ..Default::default()
            })
            .pass(PassSetting::Default)
            .primary_input_buffer(input_buffer.as_bytes())
            .tex_input_name(output_profile);

        if profile.shell_escape {
            // For now, this is the only option we allow.
            if let Some(cwd) = &profile.shell_escape_cwd {
                sess_builder.shell_escape_with_work_dir(cwd);
            } else {
                sess_builder.shell_escape_with_temp_dir();
            }
        }

        if setup_options.only_cached {
            tt_note!(status, "using only cached resource files");
        }
        sess_builder.bundle(self.bundle(setup_options, status)?);

        let mut tex_dir = self.src_dir().to_owned();
        tex_dir.push("src");
        sess_builder.filesystem_root(&tex_dir);

        let mut output_dir = self.build_dir().to_owned();
        output_dir.push(output_profile);
        ctry!(
            fs::create_dir_all(&output_dir);
            "couldn\'t create output directory `{}`", output_dir.display()
        );
        sess_builder.output_dir(&output_dir);

        Ok(sess_builder)
    }
}

/// Extension methods for [`WorkspaceCreator`].
pub trait WorkspaceCreatorExt {
    /// Create the new workspace with a good default for the bundle location.
    ///
    /// This method is a thin wrapper on [`WorkspaceCreator::create`] that uses
    /// the current configuration to determine a good default bundle location
    /// for the main document.
    fn create_defaulted(
        self,
        config: config::PersistentConfig,
        status: &mut dyn StatusBackend,
        web_bundle: Option<String>,
    ) -> Result<Workspace>;
}

impl WorkspaceCreatorExt for WorkspaceCreator {
    fn create_defaulted(
        self,
        config: config::PersistentConfig,
        status: &mut dyn StatusBackend,
        web_bundle: Option<String>,
    ) -> Result<Workspace> {
        let bundle_loc = if config::is_test_bundle_wanted(web_bundle.clone()) {
            "test-bundle://".to_owned()
        } else {
            let unresolved_loc = web_bundle.unwrap_or(config.default_bundle_loc().to_owned());
            let mut gub = DefaultBackend::default();
            gub.resolve_url(&unresolved_loc, status)?
        };

        Ok(self.create(bundle_loc, Vec::new())?)
    }
}
