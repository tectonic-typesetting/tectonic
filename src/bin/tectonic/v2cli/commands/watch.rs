use clap::Parser;
use std::{convert::Infallible, env, path::PathBuf, sync::Arc};
use tectonic::{config::PersistentConfig, errors::Result, tt_error};
use tectonic_status_base::StatusBackend;
use tokio::runtime;
use watchexec::{
    action::{Action, Outcome, PreSpawn},
    command::{Command, Shell},
    config::InitConfig,
    event::ProcessEnd,
    Watchexec,
};
use watchexec_filterer_globset::GlobsetFilterer;
use watchexec_signals::Signal;

use crate::v2cli::{CommandCustomizations, TectonicCommand};

/// Obtain the executable name without a prefix if the executable is available in the PATH, e.g.
/// most cases. Otherwise, use the full path e.g. in development.
fn get_trimmed_exe_name() -> PathBuf {
    let exe_name = env::current_exe().expect("Get current executable name");

    let path = env::var("PATH").unwrap_or_else(|_| env::var("Path").unwrap_or_default());
    let paths = env::split_paths(&path).collect::<Vec<_>>();

    for path in paths {
        if let Ok(p) = exe_name.strip_prefix(&path) {
            return p.to_owned();
        }
    }
    exe_name
}

/// `watch`: Watch input files and execute commands on change
#[derive(Debug, Eq, PartialEq, Parser)]
pub struct WatchCommand {
    /// Tectonic commands to execute on build [default: build]
    #[arg(long = "exec", short = 'x')]
    execute: Vec<String>,
}

impl WatchCommand {
    async fn execute_inner(self, status: &mut dyn StatusBackend) -> Result<i32> {
        let exe_name = get_trimmed_exe_name()
            .into_os_string()
            .into_string()
            .expect("Executable path wasn't valid UTF-8");
        let mut cmds = Vec::new();

        #[cfg(windows)]
        let shell = Shell::Cmd;
        #[cfg(unix)]
        let shell = Shell::Unix("bash".to_string());

        for x in self.execute.iter() {
            let x = x.trim();
            if !x.is_empty() {
                let cmd = Command::Shell {
                    shell: shell.clone(),
                    args: vec![],
                    command: format!("\"{exe_name}\" -X {}", x),
                };
                cmds.push(cmd)
            }
        }

        if cmds.is_empty() {
            cmds.push(Command::Exec {
                prog: exe_name,
                args: vec!["-X".to_string(), "build".to_string()],
            });
        }

        let mut runtime_config = watchexec::config::RuntimeConfig::default();
        runtime_config.commands(cmds);

        let current_dir = env::current_dir()?;

        let filter = GlobsetFilterer::new(
            &current_dir,
            [],
            // Ignore build directory, and things like vim swap files
            [("build/**".to_string(), None), ("*.swp".to_string(), None)],
            [],
            [],
        )
        .await
        .unwrap();

        runtime_config
            .pathset([&current_dir])
            .filterer(Arc::new(filter))
            .on_pre_spawn(|pre_spawn: PreSpawn| async move {
                println!("[Running `{}`]", pre_spawn.command);
                Ok::<_, Infallible>(())
            })
            .on_action(|action: Action| async move {
                for event in &*action.events {
                    let is_kill = event.signals().any(|signal| {
                        matches!(
                            signal,
                            Signal::Interrupt
                                | Signal::Quit
                                | Signal::Terminate
                                | Signal::ForceStop
                        )
                    });
                    if is_kill {
                        action.outcome(Outcome::Exit);
                        return Ok::<_, Infallible>(());
                    }

                    for complete in event.completions() {
                        match complete {
                            Some(ProcessEnd::Success) => {
                                println!("[Finished Running. Exit Status: 0]")
                            }
                            Some(ProcessEnd::ExitError(err)) => {
                                println!("[Finished Running. Exit Status: {}]", err.get())
                            }
                            _ => (),
                        }
                    }

                    let paths = event.paths().collect::<Vec<_>>();
                    if !paths.is_empty() {
                        action.outcome(Outcome::IfRunning(
                            Box::new(Outcome::DoNothing),
                            Box::new(Outcome::Start),
                        ));
                        return Ok(());
                    }
                }
                Ok(())
            });

        let exec_handler = Watchexec::new(InitConfig::default(), runtime_config);

        match exec_handler {
            Err(e) => {
                tt_error!(
                    status,
                    "failed to build arguments for watch ExecHandler";
                    e.into()
                );
                Ok(1)
            }
            Ok(exec_handler) => {
                exec_handler.main().await.unwrap().unwrap();
                Ok(0)
            }
        }
    }
}

impl TectonicCommand for WatchCommand {
    fn customize(&self, _cc: &mut CommandCustomizations) {}

    fn execute(self, _config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.execute_inner(status))
    }
}
