// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate tempdir;

use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::str;
use tempdir::TempDir;

fn prep_tectonic(cwd: &Path, args: &[&str]) -> Command {
    let tectonic = cargo_dir()
        .join("tectonic")
        .with_extension(env::consts::EXE_EXTENSION);

    match fs::metadata(&tectonic) {
        Ok(_) => {}
        Err(_) => {
            panic!("tectonic binary not found at {:?}. Do you need to run `cargo build`?",
                   tectonic)
        }
    }
    println!("using tectonic binary at {:?}", tectonic);
    println!("using cwd {:?}", cwd);

    let mut command = Command::new(tectonic);
    command.args(args);
    command.current_dir(cwd);
    command
}

fn run_tectonic(cwd: &Path, args: &[&str]) -> Output {
    let mut command = prep_tectonic(cwd, args);
    println!("running {:?}", command);
    command.output().expect("tectonic failed to start")
}

fn run_tectonic_with_stdin(cwd: &Path, args: &[&str], stdin: &str) -> Output {
    let mut command = prep_tectonic(cwd, args);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    println!("running {:?}", command);
    let mut child = command.spawn().expect("tectonic failed to start");
    write!(child.stdin.as_mut().unwrap(), "{}", stdin).expect("failed to send data to tectonic subprocess");
    child.wait_with_output().expect("failed to wait on tectonic subprocess")
}

fn setup_and_copy_files(files: &[&str]) -> TempDir {
    let tempdir = TempDir::new("tectonic_executable_test").unwrap();
    let executable_test_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("tests/executable");

    for file in files {
        // Create parent directories, if the file is not at the root of `tests/executable/`
        let file_path = PathBuf::from(file);
        let parent_dir = file_path.parent().unwrap();
        let mut dirbuilder = fs::DirBuilder::new();
        dirbuilder.recursive(true);
        dirbuilder.create(tempdir.path().join(parent_dir)).unwrap();

        fs::copy(executable_test_dir.join(file), tempdir.path().join(file)).unwrap();
    }

    tempdir
}

// Duplicated from Cargo's own testing code:
// https://github.com/rust-lang/cargo/blob/19fdb308/tests/cargotest/support/mod.rs#L305-L318
pub fn cargo_dir() -> PathBuf {
    env::var_os("CARGO_BIN_PATH")
        .map(PathBuf::from)
        .or_else(|| {
            env::current_exe()
                .ok()
                .map(|mut path| {
                         path.pop();
                         if path.ends_with("deps") {
                             path.pop();
                         }
                         path
                     })
        })
        .unwrap_or_else(|| panic!("CARGO_BIN_PATH wasn't set. Cannot continue running test"))
}

fn success_or_panic(output: Output) {
    if output.status.success() {
        println!("status: {}", output.status);
        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        panic!("Command exited badly:\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
               output.status,
               String::from_utf8_lossy(&output.stdout),
               String::from_utf8_lossy(&output.stderr));
    }
}

fn error_or_panic(output: Output) {
    if !output.status.success() {
        println!("status: {}", output.status);
        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        panic!("Command should have failed but didn't:\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
               output.status,
               String::from_utf8_lossy(&output.stdout),
               String::from_utf8_lossy(&output.stderr));
    }
}

fn check_file(tempdir: &TempDir, rest: &str) {
    let mut p = tempdir.path().to_owned();
    p.push(rest);

    if !p.is_file() {
        panic!("file \"{}\" should have been created but wasn\'t", p.to_string_lossy());
    }
}

/* Keep tests alphabetized */

#[test]
fn bad_input_path_1() {
    let output = run_tectonic(&PathBuf::from("."), &["/"]);
    error_or_panic(output);
}

#[test]
fn bad_input_path_2() {
    let output = run_tectonic(&PathBuf::from("."), &["somedir/.."]);
    error_or_panic(output);
}

#[test]
fn help_flag() {
    let output = run_tectonic(&PathBuf::from("."), &["-h"]);
    success_or_panic(output);
}

#[test] // GitHub #31
fn relative_include() {
    let tempdir = setup_and_copy_files(&["subdirectory/relative_include.tex",
                                         "subdirectory/content/1.tex"]);

    let output = run_tectonic(tempdir.path(),
                              &["--format=plain.fmt.gz", "subdirectory/relative_include.tex"]);
    success_or_panic(output);
    check_file(&tempdir, "subdirectory/relative_include.pdf");
}

#[test]
fn stdin_content() {
    // No input files here, but output files are created.
    let tempdir = setup_and_copy_files(&[]);
    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &["--format=plain", "-"],
        "Standard input content.\\bye"
    );
    success_or_panic(output);
}

// Regression #36
#[test]
fn test_space() {
    let tempdir = setup_and_copy_files(&["test space.tex"]);

    let output = run_tectonic(tempdir.path(), &["--format=plain.fmt.gz", "test space.tex"]);
    success_or_panic(output);
}

#[test] 
fn test_outdir() {
    let tempdir = setup_and_copy_files(&["subdirectory/content/1.tex"]);

    let output = run_tectonic(tempdir.path(),
                              &["--format=plain.fmt.gz", "subdirectory/content/1.tex", "--outdir=subdirectory"]);
    success_or_panic(output);
    check_file(&tempdir, "subdirectory/1.pdf");
}

#[test]
#[should_panic]
fn test_bad_outdir() {
    let tempdir = setup_and_copy_files(&["subdirectory/content/1.tex"]);

    let output = run_tectonic(tempdir.path(),
                              &["--format=plain.fmt.gz", "subdirectory/content/1.tex", "--outdir=subdirectory/non_existant"]);
    success_or_panic(output);

#[test]
fn test_keep_logs_on_error() {
    // No input files here, but output files are created.
    let tempdir = setup_and_copy_files(&[]);
    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &["--format=plain", "-", "--keep-logs"],
        "no end to this file"
    );
    error_or_panic(output);

    let mut log = String::new();
    File::open(tempdir.path().join("texput.log")).expect("`texput.log` not found")
        .read_to_string(&mut log).expect("Cannot read `texput.log`");

    assert!(log.contains(r"job aborted, no legal \end found"));
}
