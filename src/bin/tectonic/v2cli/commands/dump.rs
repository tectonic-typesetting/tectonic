use std::io::Write;

use clap::Parser;
use tectonic::{
    config::PersistentConfig,
    ctry,
    docmodel::{DocumentExt, DocumentSetupOptions},
    driver::PassSetting,
    errors::Result,
    tt_error,
};
use tectonic_bridge_core::{SecuritySettings, SecurityStance};
use tectonic_docmodel::workspace::Workspace;
use tectonic_status_base::StatusBackend;

use crate::v2cli::{CommandCustomizations, TectonicCommand};

/// `dump`: Run a partial build and dump an intermediate file
#[derive(Debug, Eq, PartialEq, Parser)]
pub struct DumpCommand {
    /// Document is untrusted -- disable all known-insecure features
    #[arg(long)]
    untrusted: bool,

    /// Use only resource files cached locally
    #[arg(short = 'C', long)]
    only_cached: bool,

    /// Use the specified output profile for the partial build
    #[arg(short = 'p', long)]
    profile: Option<String>,

    /// Dump the file or files whose names end with the argument
    #[arg(long = "suffix", short)]
    suffix_mode: bool,

    /// The name of the intermediate file to dump
    #[arg()]
    filename: String,
}

impl TectonicCommand for DumpCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
        cc.minimal_chatter = true;
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let ws = Workspace::open_from_environment()?;
        let doc = ws.first_document();

        // Default to allowing insecure since it would be super duper annoying
        // to have to pass `--trusted` every time to build a personal document
        // that uses shell-escape! This default can be overridden by setting the
        // environment variable TECTONIC_UNTRUSTED_MODE to a nonempty value.
        let stance = if self.untrusted {
            SecurityStance::DisableInsecures
        } else {
            SecurityStance::MaybeAllowInsecures
        };

        let mut setup_options =
            DocumentSetupOptions::new_with_security(SecuritySettings::new(stance));
        setup_options.only_cached(self.only_cached);

        // If output profile is unspecified, just grab one at (pseudo-)random.
        let output_name = self
            .profile
            .as_ref()
            .unwrap_or_else(|| doc.outputs.keys().next().unwrap());

        let mut builder = doc.setup_session(output_name, &setup_options, status)?;

        builder
            .format_cache_path(config.format_cache_path()?)
            .pass(PassSetting::Tex);

        let sess = crate::compile::run_and_report(builder, status)?;
        let files = sess.into_file_data();

        if self.suffix_mode {
            let mut found_any = false;

            for (key, info) in &files {
                if key.ends_with(&self.filename) {
                    found_any = true;
                    ctry!(
                        std::io::stdout().write_all(&info.data[..]);
                        "error dumping intermediate file `{}`", key
                    );
                }
            }

            if !found_any {
                tt_error!(
                    status,
                    "found no intermediate files with names ending in `{}`",
                    self.filename
                );
                return Ok(1);
            }
        } else {
            let info = files
                .get(&self.filename)
                .ok_or_else(|| format!("no such intermediate file `{}`", self.filename))?;
            ctry!(
                std::io::stdout().write_all(&info.data[..]);
                "error dumping intermediate file `{}`", self.filename
            );
        }

        Ok(0)
    }
}
