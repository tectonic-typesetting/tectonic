// Copyright 2020-2021 the Tectonic Project
// Licensed under the MIT License.

//! The "v2cli" command-line interface -- a "multitool" interface resembling
//! Cargo, as compared to the classic "rustc-like" CLI.

use std::{
    convert::Infallible, env, ffi::OsString, fs, io::Write, path::Path, path::PathBuf, process,
    str::FromStr, sync::Arc,
};
use structopt::{clap::AppSettings, StructOpt};
use tectonic::{
    self,
    config::{is_config_test_mode_activated, PersistentConfig},
    ctry,
    docmodel::{DocumentExt, DocumentSetupOptions, WorkspaceCreatorExt},
    driver::PassSetting,
    errors::{Result, SyncError},
    status::{termcolor::TermcolorStatusBackend, ChatterLevel, StatusBackend},
    tt_error, tt_note,
};
use tectonic_bridge_core::{SecuritySettings, SecurityStance};
use tectonic_bundles::Bundle;
use tectonic_docmodel::workspace::{Workspace, WorkspaceCreator};
use tectonic_errors::prelude::anyhow;
use tectonic_status_base::plain::PlainStatusBackend;
use tokio::runtime;
use watchexec::event::ProcessEnd;
use watchexec::{
    action::{Action, Outcome, PreSpawn},
    command::{Command, Shell},
    config::InitConfig,
    Watchexec,
};
use watchexec_filterer_globset::GlobsetFilterer;
use watchexec_signals::Signal;

/// The main options for the "V2" command-line interface.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "tectonic -X",
    about = "Process (La)TeX documents",
    setting(AppSettings::NoBinaryName)
)]
struct V2CliOptions {
    /// How much chatter to print when running
    #[structopt(
        long = "chatter",
        short,
        name = "level",
        default_value = "default",
        possible_values(&["default", "minimal"])
    )]
    chatter_level: String,

    /// Control colorization of output
    #[structopt(
        long = "color",
        name = "when",
        default_value = "auto",
        possible_values(&["always", "auto", "never"])
    )]
    cli_color: String,

    /// Use this URL to find resource files instead of the default
    #[structopt(
        takes_value(true),
        long,
        short,
        name = "url",
        overrides_with = "url",
        global(true)
    )]
    // TODO add URL validation
    web_bundle: Option<String>,

    #[structopt(subcommand)]
    command: Commands,
}

/// A semi-hack to allow command-specific customizations of the centralized app
/// initialization.
#[derive(Debug, Default)]
struct CommandCustomizations {
    always_stderr: bool,
    minimal_chatter: bool,
}

/// The main function for the Cargo-like, "V2" CLI. This intentionally
/// duplicates a lot of the "old" main() function, so that the implementation
/// can drift over time as needed.
pub fn v2_main(effective_args: &[OsString]) {
    // See main() -- we have a test mode that might need activating.

    tectonic::test_util::maybe_activate_test_mode();

    // I like the idea of not having any global config at all, but as we migrate
    // to the V2 CLI we'll keep it for now.

    let config = match PersistentConfig::open(false) {
        Ok(c) => c,
        Err(ref e) => {
            e.dump_uncolorized();
            process::exit(1);
        }
    };

    // Parse args -- this will exit if there are problems.

    let args = V2CliOptions::from_iter(effective_args);

    // Command-specific customizations before we do our centralized setup.
    // This is a semi-hack so that we can set up certain commands to ensure
    // that status info is always printed to stderr.

    let mut customizations = CommandCustomizations::default();
    args.command.customize(&mut customizations);

    // Set up colorized output.

    let chatter_level = if customizations.minimal_chatter {
        ChatterLevel::Minimal
    } else {
        ChatterLevel::from_str(&args.chatter_level).unwrap()
    };

    let use_cli_color = match &*args.cli_color {
        "always" => true,
        "auto" => atty::is(atty::Stream::Stdout),
        "never" => false,
        _ => unreachable!(),
    };

    let mut status = if use_cli_color {
        let mut sb = TermcolorStatusBackend::new(chatter_level);
        sb.always_stderr(customizations.always_stderr);
        Box::new(sb) as Box<dyn StatusBackend>
    } else {
        let mut sb = PlainStatusBackend::new(chatter_level);
        sb.always_stderr(customizations.always_stderr);
        Box::new(sb) as Box<dyn StatusBackend>
    };

    // For now ...

    tt_note!(
        status,
        "\"version 2\" Tectonic command-line interface activated"
    );

    // Now that we've got colorized output, pass off to the inner function.

    let code = match args.command.execute(config, &mut *status, args.web_bundle) {
        Ok(c) => c,
        Err(e) => {
            status.report_error(&SyncError::new(e).into());
            1
        }
    };

    process::exit(code)
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, StructOpt)]
enum Commands {
    #[structopt(name = "build")]
    /// Build a document
    Build(BuildCommand),

    #[structopt(name = "bundle")]
    /// Commands relating to this document’s TeX file bundle
    Bundle(BundleCommand),

    #[structopt(name = "compile")]
    /// Run a standalone (La)TeX compilation
    Compile(crate::compile::CompileOptions),

    #[structopt(name = "dump")]
    /// Run a partial compilation and output an intermediate file
    Dump(DumpCommand),

    #[structopt(name = "new")]
    /// Create a new document project
    New(NewCommand),

    #[structopt(name = "init")]
    /// Initializes a new document in the current directory
    Init(InitCommand),

    #[structopt(name = "show")]
    /// Display various useful pieces of information
    Show(ShowCommand),

    #[structopt(name = "watch")]
    /// Watch input files and execute commands on change
    Watch(WatchCommand),

    #[structopt(external_subcommand)]
    /// Runs the external command `tectonic-[command]` if one exists.
    External(Vec<String>),
}

impl Commands {
    fn customize(&self, cc: &mut CommandCustomizations) {
        match self {
            Commands::Build(o) => o.customize(cc),
            Commands::Bundle(o) => o.customize(cc),
            Commands::Compile(_) => {} // avoid namespacing/etc issues
            Commands::Dump(o) => o.customize(cc),
            Commands::New(o) => o.customize(cc),
            Commands::Init(o) => o.customize(cc),
            Commands::Show(o) => o.customize(cc),
            Commands::Watch(o) => o.customize(cc),
            Commands::External(_) => {}
        }
    }

    fn execute(
        self,
        config: PersistentConfig,
        status: &mut dyn StatusBackend,
        web_bundle: Option<String>,
    ) -> Result<i32> {
        match self {
            Commands::Build(o) => o.execute(config, status, web_bundle),
            Commands::Bundle(o) => o.execute(config, status),
            Commands::Compile(o) => o.execute(config, status, web_bundle),
            Commands::Dump(o) => o.execute(config, status),
            Commands::New(o) => o.execute(config, status, web_bundle),
            Commands::Init(o) => o.execute(config, status, web_bundle),
            Commands::Show(o) => o.execute(config, status),
            Commands::Watch(o) => o.execute(config, status),
            Commands::External(args) => do_external(args),
        }
    }
}

/// `build`: Build a document
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct BuildCommand {
    /// Document is untrusted -- disable all known-insecure features
    #[structopt(long)]
    untrusted: bool,

    /// Use only resource files cached locally
    #[structopt(short = "C", long)]
    only_cached: bool,

    /// Keep the intermediate files generated during processing
    #[structopt(short, long)]
    keep_intermediates: bool,

    /// Keep the log files generated during processing
    #[structopt(long)]
    keep_logs: bool,

    /// Print the engine's chatter during processing
    #[structopt(long = "print", short)]
    print_stdout: bool,

    /// Open built document using system handler
    #[structopt(long)]
    open: bool,

    /// Specify a target to be used by the build
    #[structopt(long, help = "Specify the target of the build.")]
    target: Option<String>,
}

impl BuildCommand {
    fn customize(&self, _cc: &mut CommandCustomizations) {}

    fn execute(
        self,
        config: PersistentConfig,
        status: &mut dyn StatusBackend,
        web_bundle: Option<String>,
    ) -> Result<i32> {
        // `--web-bundle` is not actually used for `-X build`,
        // so inform the user instead of ignoring silently.
        if let Some(url) = web_bundle {
            tt_note!(status, "--web-bundle {} ignored", &url);
            tt_note!(status, "using workspace bundle configuration");
        }
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

        for output_name in doc.output_names() {
            if let Some(out) = self.target.as_ref() {
                if out != output_name {
                    continue;
                }
            }

            let mut builder = doc.setup_session(output_name, &setup_options, status)?;

            builder
                .format_cache_path(config.format_cache_path()?)
                .keep_intermediates(self.keep_intermediates)
                .keep_logs(self.keep_logs)
                .print_stdout(self.print_stdout);

            crate::compile::run_and_report(builder, status)?;

            if self.open {
                let out_file = doc.output_main_file(output_name);

                if is_config_test_mode_activated() {
                    tt_note!(status, "not opening `{}` -- test mode", out_file.display());
                } else {
                    tt_note!(status, "opening `{}`", out_file.display());
                    if let Err(e) = open::that(&out_file) {
                        tt_error!(
                            status,
                            "failed to open `{}` with system handler",
                            out_file.display();
                            e.into()
                        )
                    }
                }
            }
        }

        Ok(0)
    }
}

/// `bundle`: Commands relating to Tectonic bundles
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct BundleCommand {
    #[structopt(subcommand)]
    command: BundleCommands,
}

#[derive(Debug, Eq, PartialEq, StructOpt)]
enum BundleCommands {
    #[structopt(name = "cat")]
    /// Dump the contents of a file in the bundle
    Cat(BundleCatCommand),

    #[structopt(name = "search")]
    /// Filter the list of filenames contained in the bundle
    Search(BundleSearchCommand),
}

impl BundleCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        match &self.command {
            BundleCommands::Cat(c) => c.customize(cc),
            BundleCommands::Search(c) => c.customize(cc),
        }
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        match self.command {
            BundleCommands::Cat(c) => c.execute(config, status),
            BundleCommands::Search(c) => c.execute(config, status),
        }
    }
}

fn get_a_bundle(
    _config: PersistentConfig,
    only_cached: bool,
    status: &mut dyn StatusBackend,
) -> Result<Box<dyn Bundle>> {
    use tectonic_docmodel::workspace::NoWorkspaceFoundError;

    match Workspace::open_from_environment() {
        Ok(ws) => {
            let doc = ws.first_document();
            let mut options: DocumentSetupOptions = Default::default();
            options.only_cached(only_cached);
            doc.bundle(&options, status)
        }

        Err(e) => {
            if e.downcast_ref::<NoWorkspaceFoundError>().is_none() {
                Err(e.into())
            } else {
                tt_note!(
                    status,
                    "not in a document workspace; using the built-in default bundle"
                );
                Ok(Box::new(tectonic_bundles::get_fallback_bundle(
                    tectonic_engine_xetex::FORMAT_SERIAL,
                    only_cached,
                    status,
                )?))
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq, StructOpt)]
struct BundleCatCommand {
    /// Use only resource files cached locally
    #[structopt(short = "C", long)]
    only_cached: bool,

    #[structopt(help = "The name of the file to dump")]
    filename: String,
}

impl BundleCatCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let mut bundle = get_a_bundle(config, self.only_cached, status)?;
        let mut ih = bundle
            .input_open_name(&self.filename, status)
            .must_exist()?;
        std::io::copy(&mut ih, &mut std::io::stdout())?;
        Ok(0)
    }
}

#[derive(Debug, Eq, PartialEq, StructOpt)]
struct BundleSearchCommand {
    /// Use only resource files cached locally
    #[structopt(short = "C", long)]
    only_cached: bool,

    #[structopt(help = "The search term")]
    term: Option<String>,
}

impl BundleSearchCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let mut bundle = get_a_bundle(config, self.only_cached, status)?;
        let files = bundle.all_files(status)?;

        // Is there a better way to do this?
        let filter: Box<dyn Fn(&str) -> bool> = if let Some(t) = self.term {
            Box::new(move |s: &str| s.contains(&t))
        } else {
            Box::new(|_: &str| true)
        };

        for filename in &files {
            if filter(filename) {
                println!("{filename}");
            }
        }

        Ok(0)
    }
}

/// `dump`: Run a partial build and dump an intermediate file
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct DumpCommand {
    /// Document is untrusted -- disable all known-insecure features
    #[structopt(long)]
    untrusted: bool,

    /// Use only resource files cached locally
    #[structopt(short = "C", long)]
    only_cached: bool,

    /// Use the specified output profile for the partial build
    #[structopt(short = "p", long)]
    profile: Option<String>,

    /// Dump the file or files whose names end with the argument
    #[structopt(long = "suffix", short)]
    suffix_mode: bool,

    /// The name of the intermediate file to dump
    #[structopt()]
    filename: String,
}

impl DumpCommand {
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

/// `watch`: Watch input files and execute commands on change
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct WatchCommand {
    /// Tectonic commands to execute on build [default: build]
    #[structopt(long = "exec", short = "x")]
    execute: Vec<String>,
}

impl WatchCommand {
    fn customize(&self, _cc: &mut CommandCustomizations) {}

    async fn execute_inner(self, status: &mut dyn StatusBackend) -> Result<i32> {
        let exe_name = crate::watch::get_trimmed_exe_name()
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

    fn execute(self, _config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let rt = runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(self.execute_inner(status))
    }
}

/// `new`: Create a new document project
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct NewCommand {
    /// The name of the document directory to create.
    #[structopt(default_value = ".")]
    path: PathBuf,
}

impl NewCommand {
    fn customize(&self, _cc: &mut CommandCustomizations) {}

    fn execute(
        self,
        config: PersistentConfig,
        status: &mut dyn StatusBackend,
        web_bundle: Option<String>,
    ) -> Result<i32> {
        tt_note!(
            status,
            "creating new document in directory `{}`",
            self.path.display()
        );

        let wc = WorkspaceCreator::new(self.path);
        ctry!(
            wc.create_defaulted(config, status, web_bundle);
            "failed to create the new Tectonic workspace"
        );
        Ok(0)
    }
}

/// `init`: Initialize a document project in the current directory.
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct InitCommand {}

impl InitCommand {
    fn customize(&self, _cc: &mut CommandCustomizations) {}

    fn execute(
        self,
        config: PersistentConfig,
        status: &mut dyn StatusBackend,
        web_bundle: Option<String>,
    ) -> Result<i32> {
        let path = env::current_dir()?;
        tt_note!(
            status,
            "creating new document in this directory ({})",
            path.display()
        );

        let wc = WorkspaceCreator::new(path);
        ctry!(
            wc.create_defaulted(config, status, web_bundle);
            "failed to create the new Tectonic workspace"
        );
        Ok(0)
    }
}

/// `show`: Show various useful pieces of information.
#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct ShowCommand {
    #[structopt(subcommand)]
    command: ShowCommands,
}

#[derive(Debug, Eq, PartialEq, StructOpt)]
enum ShowCommands {
    #[structopt(name = "user-cache-dir")]
    /// Print the location of the default per-user cache directory
    UserCacheDir(ShowUserCacheDirCommand),
}

impl ShowCommand {
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

#[derive(Debug, Eq, PartialEq, StructOpt)]
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

#[cfg(unix)]
/// On Unix, exec() to replace ourselves with the child process. This function
/// *should* never return.
fn exec_or_spawn(cmd: &mut process::Command) -> Result<i32> {
    use std::os::unix::process::CommandExt;

    // exec() only returns an io::Error directly, since on success it never
    // returns; the following tomfoolery transforms it into our Result
    // machinery as desired.
    Err(cmd.exec().into())
}

#[cfg(not(unix))]
/// On other platforms, just run the process and wait for it.
fn exec_or_spawn(cmd: &mut process::Command) -> Result<i32> {
    // code() can only return None on Unix when the subprocess was killed by a
    // signal. This function only runs if we're not on Unix, so we'll always
    // get Some.
    Ok(cmd.status()?.code().unwrap())
}

#[cfg(unix)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    use std::os::unix::prelude::*;
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(windows)]
fn is_executable<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

fn search_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Some(val) = env::var_os("PATH") {
        dirs.extend(env::split_paths(&val));
    }
    dirs
}

#[allow(clippy::redundant_closure)]
/// Run an external command by executing a subprocess.
fn do_external(all_args: Vec<String>) -> Result<i32> {
    let (cmd, args) = all_args.split_first().unwrap();

    let command_exe = format!("tectonic-{}{}", cmd, env::consts::EXE_SUFFIX);
    let path = search_directories()
        .iter()
        .map(|dir| dir.join(&command_exe))
        .find(|file| is_executable(file));

    let command = path.ok_or_else(|| {
        anyhow!(
            "no internal or external subcommand `{0}` is available (install `tectonic-{0}`?)",
            cmd
        )
    })?;

    exec_or_spawn(process::Command::new(command).args(args))
}
