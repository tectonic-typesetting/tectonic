// src/cli_driver.rs -- Command-line driver for the Tectonic engine.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate tectonic;

use clap::{Arg, App};
use std::cmp;
use std::fs::File;
use std::io::{stderr, Write};
use std::path::{Path, PathBuf};

use tectonic::engines::tex::OutputFormat;
use tectonic::errors::{Result, ResultExt};
use tectonic::io::{FilesystemIo, GenuineStdoutIo, IoProvider, IoStack, MemoryIo};
use tectonic::io::itarbundle::{HttpRangeReader, ITarBundle};
use tectonic::io::zipbundle::ZipBundle;
use tectonic::{TexEngine, TexResult, XdvipdfmxEngine};


#[repr(usize)]
#[derive(Clone, Copy, Eq, Debug)]
enum ChatterLevel {
    Minimal = 0,
    Normal,
}

impl PartialEq for ChatterLevel {
    #[inline]
    fn eq(&self, other: &ChatterLevel) -> bool {
        *self as usize == *other as usize
    }
}

impl PartialOrd for ChatterLevel {
    #[inline]
    fn partial_cmp(&self, other: &ChatterLevel) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChatterLevel {
    #[inline]
    fn cmp(&self, other: &ChatterLevel) -> cmp::Ordering {
        (*self as usize).cmp(&(*other as usize))
    }
}


struct CliIoSetup {
    pub file_bundle: Option<ZipBundle<File>>,
    pub web_bundle: Option<ITarBundle<HttpRangeReader>>,
    pub mem: MemoryIo,
    pub filesystem: FilesystemIo,
    pub genuine_stdout: Option<GenuineStdoutIo>,
}

impl CliIoSetup {
    pub fn new(file_path: Option<&str>, web_url: Option<&str>, use_genuine_stdout: bool) -> Result<CliIoSetup> {
        // I don't think we can use Option.map() because we need Result handling.
        let fb = match file_path {
            Some(p) => Some(ZipBundle::<File>::open(Path::new(&p)).chain_err(|| "error opening bundle")?),
            None => None
        };

        let wb = match web_url {
            Some(u) => Some(ITarBundle::<HttpRangeReader>::open(&u).chain_err(|| "error opening web bundle")?),
            None => None
        };

        Ok(CliIoSetup {
            mem: MemoryIo::new(true),
            filesystem: FilesystemIo::new(Path::new(""), false, true),
            file_bundle: fb,
            web_bundle: wb,
            genuine_stdout: if use_genuine_stdout {
                Some(GenuineStdoutIo::new())
            } else {
                None
            }
        })
    }

    fn as_stack<'a> (&'a mut self) -> IoStack<'a> {
        let mut providers: Vec<&mut IoProvider> = Vec::new();

        if let Some(ref mut p) = self.genuine_stdout {
            providers.push(p);
        }

        providers.push(&mut self.mem);
        providers.push(&mut self.filesystem);

        if let Some(ref mut fb) = self.file_bundle {
            providers.push(fb);
        }

        if let Some(ref mut wb) = self.web_bundle {
            providers.push(wb);
        }

        IoStack::new(providers)
    }
}


fn run() -> Result<i32> {
    let matches = App::new("Tectonic")
        .version("0.1")
        .about("Process a (La)TeX document.")
        .arg(Arg::with_name("format")
             .long("format")
             .value_name("PATH")
             .help("The \"format\" used to initialize the engine")
             .default_value("xelatex.fmt"))
        .arg(Arg::with_name("bundle")
             .long("bundle")
             .short("b")
             .value_name("PATH")
             .help("The bundle file containing LaTeX resource files")
             .takes_value(true))
        .arg(Arg::with_name("web_bundle")
             .long("web-bundle")
             .short("w")
             .value_name("URL")
             .help("The URL of a bundle file containing LaTeX resource files")
             .takes_value(true))
        .arg(Arg::with_name("outfmt")
             .long("outfmt")
             .value_name("FORMAT")
             .help("The kind of output to generate")
             .possible_values(&["pdf", "xdv"])
             .default_value("pdf"))
        .arg(Arg::with_name("keeplog")
             .long("keeplog")
             .help("Keep the \"<INPUT>.log\" file generated during processing."))
        .arg(Arg::with_name("print_stdout")
             .long("print")
             .short("p")
             .help("Print the engine's chatter during processing."))
        .arg(Arg::with_name("chatter_level")
             .long("chatter")
             .short("c")
             .value_name("LEVEL")
             .help("How much chatter to print when running")
             .possible_values(&["default", "minimal"])
             .default_value("default"))
        .arg(Arg::with_name("INPUT")
             .help("The file to process.")
             .required(true)
             .index(1))
        .get_matches ();

    let format = matches.value_of("format").unwrap();
    let input = matches.value_of("INPUT").unwrap();

    let outfmt = match matches.value_of("outfmt").unwrap() {
        "xdv" => OutputFormat::Xdv,
        "pdf" => OutputFormat::Pdf,
        _ => unreachable!()
    };

    let chatter = match matches.value_of("chatter_level").unwrap() {
        "default" => ChatterLevel::Normal,
        "minimal" => ChatterLevel::Minimal,
        _ => unreachable!()
    };

    // Set up I/O. The IoStack struct must necessarily erase types (i.e., turn
    // I/O layers into IoProvider trait objects) while it lives. But, between
    // invocations of various engines, we want to look at our individual typed
    // I/O providers and interrogate them (i.e., see what files were created
    // in the memory layer. The CliIoSetup struct helps us maintain detailed
    // knowledge of types while creating an IoStack when needed. In principle
    // we could reuse the same IoStack for each processing step, but the
    // borrow checker doesn't let us poke at (e.g.) io.mem while the IoStack
    // exists, since the IoStack keeps a mutable borrow of it.

    let mut io = CliIoSetup::new(matches.value_of("bundle"),
                                 matches.value_of("web_bundle"),
                                 matches.is_present("print_stdout"))?;

    // First TeX pass.

    let result = {
        let mut stack = io.as_stack();
        let mut engine = TexEngine::new ();
        engine.set_halt_on_error_mode (true);
        // NOTE! We manage PDF output by running the xdvipdfmx engine
        // separately, not by having the C code deal with it.
        engine.set_output_format (OutputFormat::Xdv);

        if chatter > ChatterLevel::Minimal {
            println!("Running TeX ...");
        }

        engine.process (&mut stack, format, input)
    };

    match result {
        Ok(TexResult::Spotless) => {},
        Ok(TexResult::Warnings) => {
            if chatter > ChatterLevel::Minimal {
                println!("NOTE: warnings were issued by the TeX engine; use --print and/or --keeplog for details.");
            }
        },
        Ok(TexResult::Errors) => {
            println!("NOTE: errors were issued by the TeX engine, but were ignored; \
                      use --print and/or --keeplog for details.");
        },
        Err(e) => {
            let mut s = &mut stderr();

            if let Some(output) = io.mem.files.borrow().get(io.mem.stdout_key()) {
                writeln!(s, "NOTE: the engine reported an error; its output follows:\n").expect("stderr failed");
                writeln!(s, "========================================").expect("stderr failed");
                s.write_all(output).expect("stderr failed");
                writeln!(s, "========================================").expect("stderr failed");
                writeln!(s, "").expect("stderr failed");
            }

            return Err(e);
        }
    }

    // If requested, convert the XDV output to PDF.

    if let OutputFormat::Pdf = outfmt {
        let mut xdv_path = PathBuf::from(input);
        xdv_path.set_extension("xdv");

        let mut pdf_path = PathBuf::from(input);
        pdf_path.set_extension("pdf");

        {
            let mut stack = io.as_stack();
            let mut engine = XdvipdfmxEngine::new ();

            if chatter > ChatterLevel::Minimal {
                println!("Running xdvipdfmx ...");
            }

            engine.process(&mut stack, &xdv_path.to_str().unwrap(), &pdf_path.to_str().unwrap())?;
        }

        io.mem.files.borrow_mut().remove(xdv_path.as_os_str());
    }

    // If we got this far, then we did OK. Write out the output files of
    // interest.

    for (name, contents) in &*io.mem.files.borrow() {
        let sname = name.to_string_lossy();

        if name == io.mem.stdout_key() {
            continue;
        }

        if sname.ends_with(".log") && !matches.is_present("keeplog") {
            continue;
        }

        if contents.len() == 0 {
            if chatter > ChatterLevel::Minimal {
                println!("Not writing {}: it would be empty.", sname);
            }
            continue;
        }

        if chatter > ChatterLevel::Minimal {
            println!("Writing {} ({} bytes).", sname, contents.len());
        }

        let mut f = File::create(Path::new(name))?;
        f.write_all(contents)?;
    }

    Ok(0)
}

quick_main!(run);
