// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

extern crate tempdir;

use std::env;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use std::str;
use tempdir::TempDir;

fn run_tectonic(cwd: &Path, args: &[&str]) -> Output {
    let tectonic = env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
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
    println!("running {:?}", command);

    return command.output().expect("tectonic failed to start");
}

fn setup_and_copy_files(files: &[&str]) -> TempDir {
    let tempdir = TempDir::new("tectonic_executable_test").unwrap();

    let exe = env::current_exe().unwrap();
    let root = exe.parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let executable_test_dir = root.join("tests/executable");

    for file in files {
        fs::copy(executable_test_dir.join(file), tempdir.path().join(file)).unwrap();
    }

    return tempdir;
}

fn write_output(output: &Output) {
    println!("status: {}", output.status);
    println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
}

/* Keep tests alphabetized */

#[test]
fn help_flag() {
    let tempdir = setup_and_copy_files(&[]);

    let output = run_tectonic(tempdir.path(), &["-h"]);
    write_output(&output); /* only printed on failure */
    assert!(output.status.success());
}

// Regression #36
#[test]
fn test_space() {
    let tempdir = setup_and_copy_files(&["test space.tex"]);

    let output = run_tectonic(tempdir.path(), &["--format=plain.fmt.gz", "test space.tex"]);
    write_output(&output); /* only printed on failure */
    assert!(output.status.success());
}
