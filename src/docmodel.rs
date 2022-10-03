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
    document::{BuildTargetType, Document},
    workspace::{Workspace, WorkspaceCreator},
};
use tectonic_geturl::{DefaultBackend, GetUrlBackend};
use url::Url;

use crate::{
    config, ctry,
    driver::{OutputFormat, PassSetting, ProcessingSessionBuilder},
    errors::{ErrorKind, Result},
    status::StatusBackend,
    test_util, tt_note,
};

/// Options for setting up [`Document`] instances with the driver
#[derive(Clone, Debug, Default)]
pub struct DocumentSetupOptions {
    /// Disable requests to the network, if the document’s bundle happens to be
    /// network-based.
    only_cached: bool,

    /// Security settings for engine features.
    security: SecuritySettings,
}

impl DocumentSetupOptions {
    /// Create a new set of document setup options with custom security
    /// settings.
    pub fn new_with_security(security: SecuritySettings) -> Self {
        DocumentSetupOptions {
            only_cached: false,
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

        if config::is_config_test_mode_activated() {
            Ok(Box::new(test_util::TestBundle::default()))
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
                "unrecognized output profile name \"{}\"",
                output_profile
            ))
        })?;

        let output_format = match profile.target_type {
            BuildTargetType::Html => OutputFormat::Html,
            BuildTargetType::Pdf => OutputFormat::Pdf,
        };

        let mut input_buffer = String::new();
        if !profile.preamble_file.is_empty() {
            writeln!(input_buffer, "\\input{{{}}}", profile.preamble_file)?;
        }
        if !profile.index_file.is_empty() {
            writeln!(input_buffer, "\\input{{{}}}", profile.index_file)?;
        }
        if !profile.postamble_file.is_empty() {
            writeln!(input_buffer, "\\input{{{}}}", profile.postamble_file)?;
        }

        let mut sess_builder =
            ProcessingSessionBuilder::new_with_security(setup_options.security.clone());

        sess_builder
            .output_format(output_format)
            .format_name(&profile.tex_format)
            .build_date(std::time::SystemTime::now())
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
        config: &config::PersistentConfig,
        status: &mut dyn StatusBackend,
    ) -> Result<Workspace>;
}

impl WorkspaceCreatorExt for WorkspaceCreator {
    fn create_defaulted(
        self,
        config: &config::PersistentConfig,
        status: &mut dyn StatusBackend,
    ) -> Result<Workspace> {
        let bundle_loc = if config::is_config_test_mode_activated() {
            "test-bundle://".to_owned()
        } else {
            let mut gub = DefaultBackend::default();
            gub.resolve_url(config.default_bundle_loc(), status)?
        };

        Ok(self.create(bundle_loc)?)
    }
}
