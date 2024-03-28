use clap::Parser;
use tectonic::{config::PersistentConfig, errors::Result};
use tectonic_status_base::StatusBackend;

use crate::v2cli::{CommandCustomizations, TectonicCommand};

/// `show`: Show various useful pieces of information.
#[derive(Debug, Eq, PartialEq, Parser)]
pub struct ShowCommand {
    #[command(subcommand)]
    command: ShowCommands,
}

#[derive(Debug, Eq, PartialEq, Parser)]
enum ShowCommands {
    #[command(name = "user-cache-dir")]
    /// Print the location of the default per-user cache directory
    UserCacheDir(ShowUserCacheDirCommand),
}

impl TectonicCommand for ShowCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        match &self.command {
            ShowCommands::UserCacheDir(c) => c.customize(cc),
        }
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        match self.command {
            ShowCommands::UserCacheDir(c) => c.execute(config, status),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Parser)]
struct ShowUserCacheDirCommand {}

impl ShowUserCacheDirCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
    }

    fn execute(self, _config: PersistentConfig, _status: &mut dyn StatusBackend) -> Result<i32> {
        use tectonic_bundles::cache::Cache;
        let cache = Cache::get_user_default()?;
        println!("{}", cache.root().display());
        Ok(0)
    }
}
