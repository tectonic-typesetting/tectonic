// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Decode a format file.

use clap::Parser;
use std::{fs::File, io::Read, path::PathBuf, process};
use tectonic_errors::prelude::*;
use tectonic_xetex_format::format::Format;

#[derive(Debug, Parser)]
#[clap(name = "decode", about = "Decode a Tectonic format file")]
struct Options {
    #[command(subcommand)]
    command: Commands,
}

impl Options {
    fn execute(self) -> Result<()> {
        match self.command {
            Commands::Actives(c) => c.execute_actives(),
            Commands::Catcodes(c) => c.execute_catcodes(),
            Commands::ControlSequences(c) => c.execute(),
            Commands::Strings(c) => c.execute_strings(),
        }
    }
}

#[derive(Debug, Parser)]
enum Commands {
    /// Dump the active characters
    Actives(GenericCommand),
    /// Dump the character category codes
    Catcodes(GenericCommand),
    #[command(name = "cseqs")]
    /// Dump the control sequences
    ControlSequences(CseqsCommand),
    /// Dump the strings table
    Strings(GenericCommand),
}

#[derive(Debug, Eq, PartialEq, Parser)]
struct GenericCommand {
    /// The format filename.
    #[arg()]
    path: PathBuf,
}

impl GenericCommand {
    fn parse(&self) -> Result<Format> {
        let mut file = File::open(&self.path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Format::parse(&data[..])
    }

    fn execute_actives(self) -> Result<()> {
        let fmt = self.parse()?;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        fmt.dump_actives(&mut lock)?;
        Ok(())
    }

    fn execute_catcodes(self) -> Result<()> {
        let fmt = self.parse()?;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        fmt.dump_catcodes(&mut lock)?;
        Ok(())
    }

    fn execute_strings(self) -> Result<()> {
        let fmt = self.parse()?;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        fmt.dump_string_table(&mut lock)?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Parser)]
struct CseqsCommand {
    /// Whether to dump extended information such as macro contents
    #[arg(long = "extended", short = 'e')]
    extended: bool,

    /// The format filename.
    #[arg()]
    path: PathBuf,
}

impl CseqsCommand {
    fn parse(&self) -> Result<Format> {
        let mut file = File::open(&self.path)?;
        let mut data = Vec::new();
        file.read_to_end(&mut data)?;
        Format::parse(&data[..])
    }

    fn execute(self) -> Result<()> {
        let fmt = self.parse()?;
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        fmt.dump_cseqs(&mut lock, self.extended)?;
        Ok(())
    }
}

fn main() {
    let options = Options::parse();

    if let Err(e) = options.execute() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
