use clap::{CommandFactory, Parser};
use tectonic::{config::PersistentConfig, errors::Result};
use tectonic_io_base::app_dirs;
use tectonic_status_base::StatusBackend;

use crate::v2cli::{CommandCustomizations, TectonicCommand, V2CliOptions};

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

    #[command(name = "shell-completions")]
    /// Print shell completions code for some given shell
    ShellCompletions(ShowShellCompletionsCommand),
}

impl TectonicCommand for ShowCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        match &self.command {
            ShowCommands::UserCacheDir(c) => c.customize(cc),
            ShowCommands::ShellCompletions(c) => c.customize(cc),
        }
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        match self.command {
            ShowCommands::UserCacheDir(c) => c.execute(config, status),
            ShowCommands::ShellCompletions(c) => c.execute(config, status),
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
        println!("{}", app_dirs::get_user_cache_dir("bundles")?.display());
        Ok(0)
    }
}

#[derive(Debug, Eq, PartialEq, Parser)]
struct ShowShellCompletionsCommand {
    /// Target shell for the generated completion code
    shell: clap_complete::Shell,
}

impl ShowShellCompletionsCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
    }

    /// Generates shell completions at runtime
    fn execute(self, _config: PersistentConfig, _status: &mut dyn StatusBackend) -> Result<i32> {
        // The current v1 & v2 cli mixture makes it a bit difficult to offer
        // clean completions for the `tectonic` command, so for now we only
        // target the `nextonic` command exclusively for the v2 cli.
        clap_complete::generate(
            self.shell,
            &mut V2CliOptions::command(),
            "nextonic",
            &mut std::io::stdout(),
        );
        Ok(0)
    }
}
