use clap::Parser;
use std::time::Duration;
use std::{env, path::PathBuf, sync::Arc};
use tectonic::{config::PersistentConfig, errors::Result, tt_error};
use tectonic_status_base::StatusBackend;
use tokio::runtime;
use watchexec::command::Program;
use watchexec::{
    command::{Command, Shell},
    Id, Watchexec,
};
use watchexec_filterer_globset::GlobsetFilterer;
use watchexec_signals::Signal;
use watchexec_supervisor::job::{CommandState, Job, Ticket};
use watchexec_supervisor::ProcessEnd;

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

        let v2cli_enabled = exe_name.contains("nextonic");

        #[cfg(windows)]
        let shell = Shell::cmd();
        #[cfg(unix)]
        let shell = Shell::new("bash");

        for x in self.execute.iter() {
            let x = x.trim();
            if !x.is_empty() {
                let command = if v2cli_enabled {
                    format!("\"{exe_name}\" {}", x)
                } else {
                    format!("\"{exe_name}\" -X {}", x)
                };

                let cmd = Command {
                    program: Program::Shell {
                        shell: shell.clone(),
                        command,
                        args: vec![],
                    },
                    options: Default::default(),
                };
                cmds.push((Id::default(), Arc::new(cmd)))
            }
        }

        if cmds.is_empty() {
            let mut args = Vec::with_capacity(2);

            if !v2cli_enabled {
                args.push("-X".to_string());
            }
            args.push("build".to_string());

            let cmd = Command {
                program: Program::Exec {
                    prog: exe_name.into(),
                    args,
                },
                options: Default::default(),
            };

            cmds.push((Id::default(), Arc::new(cmd)));
        }

        let current_dir = env::current_dir()?;

        let filter = GlobsetFilterer::new(
            &current_dir,
            [],
            // Ignore build directory, and things like vim swap files
            [("build/**".to_string(), None), ("*.swp".to_string(), None)],
            [],
            [],
            [],
        )
        .await
        .unwrap();

        async fn end_task(end: Ticket, job: Job) {
            end.await;
            job.run(|ctx| match ctx.current {
                CommandState::Finished {
                    status: ProcessEnd::Success,
                    ..
                } => {
                    println!("[Finished Running. Exit Status: 0]")
                }
                CommandState::Finished {
                    status: ProcessEnd::ExitError(err),
                    ..
                } => {
                    println!("[Finished Running. Exit Status: {}]", err.get())
                }
                _ => (),
            })
            .await;
        }

        let cmds = Arc::new(cmds);
        let exec_handler = Watchexec::new_async(move |mut action| {
            let cmds = Arc::clone(&cmds);
            Box::new(async move {
                // When we spawn a job it doesn't immediately become available. So we chain it
                // with existing jobs.
                let mut new_job = None;

                if action.get_job(cmds[0].0).is_none() {
                    for (id, cmd) in &*cmds {
                        let job = action.get_or_create_job(*id, || Arc::clone(cmd));
                        job.set_spawn_hook(|_, ctx| {
                            println!("[Running `{}`]", ctx.command);
                        });
                        new_job = Some((*id, job));
                    }
                }

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
                        // Give the jobs a quit signal, then a short time to clean themselves up
                        action.quit_gracefully(Signal::Quit, Duration::from_millis(100));
                        return action;
                    }

                    let paths = event.paths().collect::<Vec<_>>();
                    if !paths.is_empty() {
                        for (_, job) in action.list_jobs().chain(new_job) {
                            job.start().await;
                            let end = job.to_wait();
                            tokio::spawn(end_task(end, job));
                        }
                        return action;
                    }
                }
                action
            })
        });

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
                exec_handler
                    .config
                    .pathset([current_dir])
                    .filterer(Arc::new(filter));
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
