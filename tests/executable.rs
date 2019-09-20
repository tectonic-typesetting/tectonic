// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

use lazy_static::lazy_static;

use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::str;
use tempfile::TempDir;

#[path = "util/mod.rs"]
mod util;
use crate::util::{cargo_dir, ensure_plain_format};

lazy_static! {
    static ref TEST_ROOT: PathBuf = {
        util::set_test_root();

        let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        root.push("tests");
        root
    };
}

fn get_plain_format_arg() -> String {
    util::set_test_root();
    let path = ensure_plain_format().expect("couldn't write format file");
    format!("--format={}", path.display())
}

/// Note the special sauce here — we set the magic environment variable that
/// tells the Tectonic binary to go into "test mode" and use local test
/// assets, rather than an actual network bundle.
fn prep_tectonic(cwd: &Path, args: &[&str]) -> Command {
    let tectonic = cargo_dir()
        .join("tectonic")
        .with_extension(env::consts::EXE_EXTENSION);

    if fs::metadata(&tectonic).is_err() {
        panic!(
            "tectonic binary not found at {:?}. Do you need to run `cargo build`?",
            tectonic
        )
    }
    println!("using tectonic binary at {:?}", tectonic);
    println!("using cwd {:?}", cwd);

    let mut command = Command::new(tectonic);
    command.args(args).current_dir(cwd).env(
        tectonic::test_util::TEST_ROOT_ENV_VAR,
        TEST_ROOT.as_os_str(),
    );
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
    write!(child.stdin.as_mut().unwrap(), "{}", stdin)
        .expect("failed to send data to tectonic subprocess");
    child
        .wait_with_output()
        .expect("failed to wait on tectonic subprocess")
}

fn setup_and_copy_files(files: &[&str]) -> TempDir {
    let tempdir = tempfile::Builder::new()
        .prefix("tectonic_executable_test")
        .tempdir()
        .unwrap();
    let executable_test_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/executable");

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

fn success_or_panic(output: Output) {
    if output.status.success() {
        println!("status: {}", output.status);
        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        panic!(
            "Command exited badly:\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn error_or_panic(output: Output) {
    if !output.status.success() {
        println!("status: {}", output.status);
        println!("stdout:\n{}", String::from_utf8_lossy(&output.stdout));
        println!("stderr:\n{}", String::from_utf8_lossy(&output.stderr));
    } else {
        panic!(
            "Command should have failed but didn't:\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
            output.status,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn check_file(tempdir: &TempDir, rest: &str) {
    let mut p = tempdir.path().to_owned();
    p.push(rest);

    if !p.is_file() {
        panic!(
            "file \"{}\" should have been created but wasn\'t",
            p.to_string_lossy()
        );
    }
}

/* Keep tests alphabetized */

#[test]
fn bad_chatter_1() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let output = run_tectonic(&PathBuf::from("."), &["-", "--chatter=reticent"]);
    error_or_panic(output);
}

#[test]
fn bad_input_path_1() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let output = run_tectonic(&PathBuf::from("."), &["/"]);
    error_or_panic(output);
}

#[test]
fn bad_input_path_2() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let output = run_tectonic(&PathBuf::from("."), &["somedir/.."]);
    error_or_panic(output);
}

#[test]
fn bad_outfmt_1() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let output = run_tectonic(&PathBuf::from("."), &["-", "--outfmt=dd"]);
    error_or_panic(output);
}

#[test]
fn help_flag() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let output = run_tectonic(&PathBuf::from("."), &["-h"]);
    success_or_panic(output);
}

#[test] // GitHub #31
fn relative_include() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[
        "subdirectory/relative_include.tex",
        "subdirectory/content/1.tex",
    ]);

    let output = run_tectonic(
        tempdir.path(),
        &[&fmt_arg, "subdirectory/relative_include.tex"],
    );
    success_or_panic(output);
    check_file(&tempdir, "subdirectory/relative_include.pdf");
}

#[test]
fn stdin_content() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    // No input files here, but output files are created.
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);
    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-"],
        "Standard input content.\\bye",
    );
    success_or_panic(output);
}

// Regression #36
#[test]
fn test_space() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&["test space.tex"]);

    let output = run_tectonic(tempdir.path(), &[&fmt_arg, "test space.tex"]);
    success_or_panic(output);
}

#[test]
fn test_outdir() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&["subdirectory/content/1.tex"]);

    let output = run_tectonic(
        tempdir.path(),
        &[
            &fmt_arg,
            "subdirectory/content/1.tex",
            "--outdir=subdirectory",
        ],
    );
    success_or_panic(output);
    check_file(&tempdir, "subdirectory/1.pdf");
}

#[test]
#[should_panic]
fn test_bad_outdir() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        panic!()
    }

    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&["subdirectory/content/1.tex"]);

    let output = run_tectonic(
        tempdir.path(),
        &[
            &fmt_arg,
            "subdirectory/content/1.tex",
            "--outdir=subdirectory/non_existent",
        ],
    );
    success_or_panic(output);
}

#[test]
#[should_panic]
fn test_outdir_is_file() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        panic!()
    }

    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&["test space.tex", "subdirectory/content/1.tex"]);

    let output = run_tectonic(
        tempdir.path(),
        &[
            &fmt_arg,
            "subdirectory/content/1.tex",
            "--outdir=test space.tex",
        ],
    );
    success_or_panic(output);
}

#[test]
fn test_keep_logs_on_error() {
    if env::var("RUNNING_COVERAGE").is_ok() {
        return;
    }

    // No input files here, but output files are created.
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);
    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "--keep-logs"],
        "no end to this file",
    );
    error_or_panic(output);

    let mut log = String::new();
    File::open(tempdir.path().join("texput.log"))
        .expect("`texput.log` not found")
        .read_to_string(&mut log)
        .expect("Cannot read `texput.log`");

    assert!(log.contains(r"job aborted, no legal \end found"));
}
