use clap::{Parser, ValueEnum};
use std::{fmt::Display, path::PathBuf};
use tectonic::{config::PersistentConfig, Result};
use tectonic_status_base::StatusBackend;
use tracing::error;

use crate::v2cli::{CommandCustomizations, TectonicCommand};

//
// MARK: Cli arguments
//

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum BundleJob {
    /// Run the following jobs in order
    #[value(name = "all")]
    All,

    /// (Stage 1) Select and patch all files in this bundle
    #[value(name = "select")]
    Select,

    /// (Stage 2) Pack selected files into a bundle
    #[value(name = "pack")]
    Pack,
}

impl Display for BundleJob {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Select => write!(f, "select"),
            Self::Pack => write!(f, "pack"),
        }
    }
}

impl BundleJob {
    pub fn do_select(&self) -> bool {
        matches!(self, Self::All | Self::Select)
    }

    pub fn do_pack(&self) -> bool {
        matches!(self, Self::All | Self::Pack)
    }
}

#[derive(Parser, Debug)]
pub struct BundleCreateCommand {
    /// Which job we should run. `all` is default,
    /// but single jobs can be run on their own for debugging.
    #[arg(long, default_value_t = BundleJob::All)]
    pub job: BundleJob,

    /// Bundle specification TOML file.
    pub bundle_spec: PathBuf,

    /// Build directory for this bundle.
    /// Will be removed.
    #[arg(long)]
    pub build_dir: PathBuf,

    /// What kind of bundle should we produce?
    /// This only has an effect when running jobs `all` or `pack`
    #[arg(default_value_t = BundleFormat::BundleV1)]
    pub format: BundleFormat,

    /// If this flag is set, don't fail when an input's hash doesn't match
    /// the hash specified in the bundle's configuration file.
    /// This only has an effect when running jobs `all` or `select`
    #[arg(long, default_value_t = false)]
    pub allow_hash_mismatch: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum BundleFormat {
    #[value(name = "v1")]
    BundleV1,
}

impl Display for BundleFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BundleV1 => write!(f, "v1")?,
        }
        Ok(())
    }
}

impl TectonicCommand for BundleCreateCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
    }

    fn execute(self, _config: PersistentConfig, _status: &mut dyn StatusBackend) -> Result<i32> {
        if self.job.do_select() {
            match super::actions::select(&self) {
                Ok(_) => {}
                Err(e) => {
                    error!("select job failed with error: {e}");
                    return Err(e);
                }
            };
        }

        if self.job.do_pack() {
            match super::actions::pack(&self) {
                Ok(_) => {}
                Err(e) => {
                    error!("bundle packer failed with error: {e}");
                    return Err(e);
                }
            };
        }

        Ok(0)
    }
}
