use clap::{Parser, Subcommand};
use std::io::Write;
use tectonic::{
    config::PersistentConfig,
    docmodel::{DocumentExt, DocumentSetupOptions},
    errors::Result,
    io::{InputFeatures, OpenResult},
    tt_note,
};
use tectonic_bundles::Bundle;
use tectonic_docmodel::workspace::Workspace;
use tectonic_status_base::StatusBackend;

use crate::v2cli::{CommandCustomizations, TectonicCommand};

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

/// `bundle`: Commands relating to Tectonic bundles
#[derive(Debug, Eq, PartialEq, Parser)]
pub struct BundleCommand {
    #[command(subcommand)]
    command: BundleCommands,
}

#[derive(Debug, Eq, PartialEq, Subcommand)]
enum BundleCommands {
    #[command(name = "cat")]
    /// Dump the contents of a file in the bundle
    Cat(BundleCatCommand),

    #[command(name = "search")]
    /// Filter the list of filenames contained in the bundle
    Search(BundleSearchCommand),

    #[command(name = "serve")]
    /// Dump the contents of files requested on standard input
    Serve(BundleServeCommand),
}

impl TectonicCommand for BundleCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        match &self.command {
            BundleCommands::Cat(c) => c.customize(cc),
            BundleCommands::Search(c) => c.customize(cc),
            BundleCommands::Serve(c) => c.customize(cc),
        }
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        match self.command {
            BundleCommands::Cat(c) => c.execute(config, status),
            BundleCommands::Search(c) => c.execute(config, status),
            BundleCommands::Serve(c) => c.execute(config, status),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Parser)]
struct BundleCatCommand {
    /// Use only resource files cached locally
    #[arg(short = 'C', long)]
    only_cached: bool,

    #[arg(help = "The name of the file to dump")]
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

#[derive(Debug, Eq, PartialEq, Parser)]
struct BundleSearchCommand {
    /// Use only resource files cached locally
    #[arg(short = 'C', long)]
    only_cached: bool,

    #[arg(help = "The search term")]
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

#[derive(Debug, Eq, PartialEq, Parser)]
struct BundleServeCommand {
    /// Use only resource files cached locally
    #[arg(short = 'C', long)]
    only_cached: bool,
}

impl BundleServeCommand {
    fn customize(&self, cc: &mut CommandCustomizations) {
        cc.always_stderr = true;
    }

    fn execute(self, config: PersistentConfig, status: &mut dyn StatusBackend) -> Result<i32> {
        let mut bundle = get_a_bundle(config, self.only_cached, status)?;
        let mut filename = String::new();
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        loop {
            stdout.flush()?;
            filename.clear();
            match stdin.read_line(&mut filename) {
                Err(error) => {
                    eprintln!("error: {error}");
                    return Ok(1);
                }
                Ok(0) => return Ok(0),
                Ok(_) => {
                    let name = filename.trim_end();
                    let error = match bundle.input_path(name, status) {
                        OpenResult::Err(e) => e,
                        OpenResult::Ok(path) => {
                            let mut path = path.as_os_str().as_encoded_bytes();
                            let size = path.len() as u64;
                            stdout.write_all(&[b'P'])?;
                            stdout.write_all(&size.to_le_bytes())?;
                            let copied = std::io::copy(&mut path, &mut stdout)?;
                            assert!(size == copied);
                            continue;
                        }
                        OpenResult::NotAvailable => {
                            match bundle.input_open_name(name, status).must_exist() {
                                Ok(mut t) => {
                                    let size = t.get_size()? as u64;
                                    stdout.write_all(&[b'C'])?;
                                    stdout.write_all(&size.to_le_bytes())?;
                                    let copied = std::io::copy(&mut t, &mut stdout)?;
                                    assert!(size == copied);
                                    continue;
                                }
                                Err(e) => e,
                            }
                        }
                    };
                    let text = error.to_string();
                    let bytes = text.as_bytes();
                    let size = bytes.len() as u64;
                    stdout.write_all(&[b'E'])?;
                    stdout.write_all(&size.to_le_bytes())?;
                    stdout.write_all(&bytes)?;
                }
            };
            stdout.flush().unwrap();
        }
    }
}
