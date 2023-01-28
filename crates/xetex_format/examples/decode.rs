// Copyright 2021 the Tectonic Project
// Licensed under the MIT License.

//! Decode a format file.

use std::{fs::File, io::Read, path::PathBuf, process};
use structopt::StructOpt;
use tectonic_errors::prelude::*;
use tectonic_xetex_format::format::Format;

#[derive(Debug, StructOpt)]
#[structopt(name = "decode", about = "Decode a Tectonic format file")]
struct Options {
    #[structopt(subcommand)]
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

#[derive(Debug, StructOpt)]
enum Commands {
    #[structopt(name = "actives")]
    /// Dump the active characters
    Actives(GenericCommand),

    #[structopt(name = "catcodes")]
    /// Dump the character category codes
    Catcodes(GenericCommand),

    #[structopt(name = "cseqs")]
    /// Dump the control sequences
    ControlSequences(CseqsCommand),

    #[structopt(name = "strings")]
    /// Dump the strings table
    Strings(GenericCommand),
}

#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct GenericCommand {
    /// The format filename.
    #[structopt()]
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

#[derive(Debug, Eq, PartialEq, StructOpt)]
pub struct CseqsCommand {
    /// Whether to dump extended information such as macro contents
    #[structopt(long = "extended", short = "e")]
    extended: bool,

    /// The format filename.
    #[structopt()]
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
    let options = Options::from_args();

    if let Err(e) = options.execute() {
        eprintln!("error: {e}");
        process::exit(1);
    }
}
