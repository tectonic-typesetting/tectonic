// Licensed under the MIT License.

//! Test suite for the top-level `tectonic` executable.

use lazy_static::lazy_static;
use std::io::ErrorKind;
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
    str, thread,
    time::{Duration, Instant},
};
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

    static ref TARGET_RUNNER_WORDS: Vec<String> = {
        // compile-time environment variable from build.rs:
        let target = env!("TARGET").to_owned();
        let mut target = target.replace('-', "_");
        target.make_ascii_uppercase();

        // run-time environment variable check:
        if let Ok(runtext) = env::var(format!("CARGO_TARGET_{target}_RUNNER")) {
            runtext.split_whitespace().map(|x| x.to_owned()).collect()
        } else {
            vec![]
        }
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
        panic!("tectonic binary not found at {tectonic:?}. Do you need to run `cargo build`?")
    }
    println!("using tectonic binary at {tectonic:?}");
    println!("using cwd {cwd:?}");

    // We may need to wrap the Tectonic invocation. If we're cross-compiling, we
    // might need to use something like QEMU to actually be able to run the
    // executable.
    let mut command = if !TARGET_RUNNER_WORDS.is_empty() {
        let mut cmd = Command::new(&TARGET_RUNNER_WORDS[0]);
        cmd.args(&TARGET_RUNNER_WORDS[1..]).arg(tectonic);
        cmd
    } else {
        Command::new(tectonic)
    };

    command.args(args).current_dir(cwd).env(
        tectonic::test_util::TEST_ROOT_ENV_VAR,
        TEST_ROOT.as_os_str(),
    );
    command
}

fn run_tectonic(cwd: &Path, args: &[&str]) -> Output {
    let mut command = prep_tectonic(cwd, args);
    command.env("BROWSER", "echo");
    println!("running {command:?}");
    command.output().expect("tectonic failed to start")
}

fn run_tectonic_until(cwd: &Path, args: &[&str], mut kill: impl FnMut() -> bool) -> Output {
    let mut command = prep_tectonic(cwd, args);
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    command.env("BROWSER", "echo");

    println!("running {command:?} until test passes");
    let mut child = command.spawn().expect("tectonic failed to start");
    while !kill() {
        thread::sleep(Duration::from_secs(1));
    }

    // Ignore if the child already died
    // TODO: This causes coverage to not be reported
    let _ = child.kill();
    child
        .wait_with_output()
        .expect("tectonic failed to execute")
}

fn run_tectonic_with_stdin(cwd: &Path, args: &[&str], stdin: &str) -> Output {
    let mut command = prep_tectonic(cwd, args);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    println!("running {command:?}");
    let mut child = command.spawn().expect("tectonic failed to start");
    match write!(child.stdin.as_mut().unwrap(), "{stdin}") {
        Ok(_) => (),
        // Ignore if the child already died
        Err(e) if e.kind() == ErrorKind::BrokenPipe => (),
        Err(e) => panic!("failed to send data to tectonic subprocess: {e:?}"),
    }
    child
        .wait_with_output()
        .expect("failed to wait on tectonic subprocess")
}

fn setup_and_copy_files(files: &[&str]) -> TempDir {
    let tempdir = tempfile::Builder::new()
        .prefix("tectonic_executable_test")
        .tempdir()
        .unwrap();

    let executable_test_dir = if let Some(v) = env::var_os("CARGO_MANIFEST_DIR") {
        PathBuf::from(v)
    } else {
        PathBuf::new()
    }
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

fn success_or_panic(output: &Output) {
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

fn error_or_panic(output: &Output) {
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

fn setup_v2() -> (tempfile::TempDir, PathBuf) {
    util::set_test_root();

    let tempdir = setup_and_copy_files(&[]);
    let mut temppath = tempdir.path().to_owned();
    let output = run_tectonic(&temppath, &["-X", "new", "doc"]);
    success_or_panic(&output);

    temppath.push("doc");

    // To run a build in our test setup, we can only use plain TeX. So, jankily
    // change the format ...

    {
        let mut toml_path = temppath.clone();
        toml_path.push("Tectonic.toml");
        let mut file = OpenOptions::new().append(true).open(toml_path).unwrap();
        writeln!(file, "tex_format = 'plain'").unwrap();
    }

    // ... and write some files that are plain TeX.

    {
        let mut path = temppath.clone();
        path.push("src");

        {
            path.push("_preamble.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file).unwrap();
            path.pop();
        }

        {
            path.push("_postamble.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file, "\\end").unwrap();
            path.pop();
        }
    }

    (tempdir, temppath)
}

/* Keep tests alphabetized */

#[test]
fn bad_chatter_1() {
    let output = run_tectonic(&PathBuf::from("."), &["-", "--chatter=reticent"]);
    error_or_panic(&output);
}

#[test]
fn bad_input_path_1() {
    let output = run_tectonic(&PathBuf::from("."), &["/"]);
    error_or_panic(&output);
}

#[test]
fn bad_input_path_2() {
    let output = run_tectonic(&PathBuf::from("."), &["somedir/.."]);
    error_or_panic(&output);
}

#[test]
fn bad_outfmt_1() {
    let output = run_tectonic(&PathBuf::from("."), &["-", "--outfmt=dd"]);
    error_or_panic(&output);
}

fn run_with_biber(args: &str, stdin: &str) -> Output {
    run_with_biber_exe(None, args, stdin, &["subdirectory/empty.bib"])
}

fn run_with_biber_exe(executable: Option<&str>, args: &str, stdin: &str, files: &[&str]) -> Output {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(files);
    let mut command = prep_tectonic(tempdir.path(), &[&fmt_arg, "-"]);

    let test_cmd = if let Some(exe) = executable {
        format!("{exe} {args}")
    } else if cfg!(windows) {
        format!(
            "cmd /c {} {}",
            util::test_path(&["fake-biber.bat"]).display(),
            args
        )
    } else {
        format!("{} {}", util::test_path(&["fake-biber.sh"]).display(), args)
    };

    command.env("TECTONIC_TEST_FAKE_BIBER", &test_cmd);

    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    println!("running {command:?}");
    let mut child = command.spawn().expect("tectonic failed to start");

    write!(child.stdin.as_mut().unwrap(), "{stdin}")
        .expect("failed to send data to tectonic subprocess");

    child
        .wait_with_output()
        .expect("failed to wait on tectonic subprocess")
}

const BIBER_TRIGGER_TEX: &str = r#"
% Test if we're on the second pass by seeing if the BCF already exists
\newif\ifsecond
\newread\r
\openin\r=texput.bcf
\ifeof\r
\message{first pass}
\secondfalse
\else
\message{second pass}
\secondtrue
\closein\r
\fi

% Now create BCF
\newwrite\w
\immediate\openout\w=texput.bcf\relax
\immediate\write\w{Hello BCF file}
\immediate\closeout\w

% Now create run.xml
\immediate\openout\w=texput.run.xml\relax
\immediate\write\w{
<requests version="1.0">
    <external package="biblatex" priority="5" active="1">
        <generic>biber</generic>
        <cmdline>
            <binary>biber</binary>
            <infile>texput</infile>
        </cmdline>
        <input>
            <file>texput.bcf</file>
        </input>
        <output>
            <file>texput.bbl</file>
        </output>
        <provides type="dynamic">
            <file>texput.bbl</file>
        </provides>
        <requires type="dynamic">
            <file>texput.bcf</file>
        </requires>
        <requires type="editable">
            <file>subdirectory/empty.bib</file>
        </requires>
    </external>
</requests>
}
\immediate\closeout\w
"#;

#[test]
fn biber_failure() {
    let output = run_with_biber("failure", BIBER_TRIGGER_TEX);
    error_or_panic(&output);
}

#[test]
fn biber_no_such_tool() {
    const REST: &str = r"\bye";
    let tex = format!("{BIBER_TRIGGER_TEX}{REST}");
    let output = run_with_biber_exe(Some("ohnothereisnobiberprogram"), "", &tex, &[]);
    error_or_panic(&output);
}

#[cfg(unix)]
#[test]
fn biber_signal() {
    let output = run_with_biber("signal", BIBER_TRIGGER_TEX);
    error_or_panic(&output);
}

const BIBER_VALIDATE_TEX: &str = r"
\ifsecond
\ifnum\input{biberout.qqq}=456\relax
a
\else
\ohnothebiberdidntwork
\fi
\fi
\bye";

#[test]
fn biber_success() {
    let tex = format!("{BIBER_TRIGGER_TEX}{BIBER_VALIDATE_TEX}");
    let output = run_with_biber("success", &tex);
    success_or_panic(&output);
}

/// Test `tectonic-biber` override: when no args passed, fall back to $PATH
/// lookup for `tectonic-biber` first, and then `biber`. Currently defined in:
/// [`tectonic::driver::ProcessingSession::check_biber_requirement`]
#[cfg(unix)]
#[test]
fn biber_tectonic_override() {
    let tex = format!("{BIBER_TRIGGER_TEX}{BIBER_VALIDATE_TEX}");
    let output = run_with_biber_exe(
        Some(""),
        "", // no args passed
        &tex,
        &["subdirectory/empty.bib", "tectonic-biber"],
    );
    success_or_panic(&output);
}

/// #844: biber input with absolute path blows away the file
///
/// We need to create a separate temporary directory to see if the abspath input
/// gets messed up.
///
/// Backslashes in Windows paths mess up our naive test. I can't figure out how
/// to get them to work here (admittedly, not trying too hard) so I'm just
/// skipping this test on that platform.
#[test]
#[cfg(not(windows))]
fn biber_issue_844() {
    let temp_source = setup_and_copy_files(&[]);
    let mut bibpath = std::fs::canonicalize(temp_source.path()).unwrap();
    bibpath.push("single_entry.bib");

    let contents = include_str!("bibtex/cites/single_entry.bib");
    std::fs::write(&bibpath, contents.as_bytes()).unwrap();

    // Futz the basic template to reference our absolute path input file:
    let tex = format!(
        "{}{}",
        BIBER_TRIGGER_TEX.replace(">texput.bcf<", &format!(">{}<", bibpath.to_str().unwrap())),
        "kthx\\bye"
    );

    let output = run_with_biber("success", &tex);
    success_or_panic(&output);

    let stat = std::fs::metadata(&bibpath).unwrap();
    assert_eq!(stat.len(), contents.len() as u64);
}

/// Fakily testing the bibtex implementation even though we don't have LaTeX.
/// This implementation is strongly cribbed from the biber tests, which came
/// first.
#[test]
fn bibtex_multiple_aux_files() {
    let fmt_arg = get_plain_format_arg();
    let tempdir =
        setup_and_copy_files(&["bibtex_multiple_aux_files.tex", "refs.bib", "catchkey.bst"]);
    let output = run_tectonic(tempdir.path(), &[&fmt_arg, "bibtex_multiple_aux_files.tex"]);
    success_or_panic(&output);
}

#[test]
fn help_flag() {
    let output = run_tectonic(&PathBuf::from("."), &["-h"]);
    success_or_panic(&output);
}

#[test]
fn keep_logs_on_error() {
    // No input files here, but output files are created.
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);
    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "--keep-logs"],
        "no end to this file",
    );
    error_or_panic(&output);

    let mut log = String::new();
    File::open(tempdir.path().join("texput.log"))
        .expect("`texput.log` not found")
        .read_to_string(&mut log)
        .expect("Cannot read `texput.log`");

    assert!(log.contains(r"job aborted, no legal \end found"));
}

#[test]
fn no_color_option() {
    // No input files here, but output files are created.
    let fmt_arg = get_plain_format_arg();

    let tempdir = setup_and_copy_files(&[]);
    let output_nocolor = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "--color=never"],
        "no end to this file",
    );

    // Output is not a terminal, so these two should be the same
    let tempdir = setup_and_copy_files(&[]);
    let output_autocolor = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "--color=auto"],
        "no end to this file",
    );

    assert_eq!(output_nocolor, output_autocolor);

    error_or_panic(&output_nocolor);
    error_or_panic(&output_autocolor);
}

#[test]
fn outdir_option() {
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
    success_or_panic(&output);
    check_file(&tempdir, "subdirectory/1.pdf");
}

#[test]
#[should_panic]
// panic unwinding broken: https://github.com/rust-embedded/cross/issues/343
#[cfg(not(all(target_arch = "arm", target_env = "musl")))]
fn outdir_option_bad() {
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
    success_or_panic(&output);
}

#[test]
#[should_panic]
// panic unwinding broken: https://github.com/rust-embedded/cross/issues/343
#[cfg(not(all(target_arch = "arm", target_env = "musl")))]
fn outdir_option_is_file() {
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
    success_or_panic(&output);
}

#[test] // GitHub #31
fn relative_include() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[
        "subdirectory/relative_include.tex",
        "subdirectory/content/1.tex",
    ]);

    let output = run_tectonic(
        tempdir.path(),
        &[&fmt_arg, "subdirectory/relative_include.tex"],
    );
    success_or_panic(&output);
    check_file(&tempdir, "subdirectory/relative_include.pdf");
}

// Regression #36
#[test]
fn space_in_filename() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&["test space.tex"]);

    let output = run_tectonic(tempdir.path(), &[&fmt_arg, "test space.tex"]);
    success_or_panic(&output);
}

#[test]
fn stdin_content() {
    // No input files here, but output files are created.
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);
    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-"],
        "Standard input content.\\bye",
    );
    success_or_panic(&output);
}

/// Test various web bundle overrides for the v1 CLI & `-X compile`
#[test]
fn bundle_overrides() {
    let filename = "subdirectory/content/1.tex";
    let fmt_arg: &str = &get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[filename]);
    let temppath = tempdir.path().to_owned();

    let arg_bad_bundle = ["--bundle", "bad-bundle"];
    let arg_good_bundle = ["--bundle", "test-bundle://"];

    // test with a bad bundle
    let output = run_tectonic(
        &temppath,
        &[&arg_bad_bundle[..], &[fmt_arg, filename]].concat(),
    );
    error_or_panic(&output);

    // test with a good bundle (override)
    let mut valid_args: Vec<Vec<&str>> = vec![
        // different positions
        [&arg_good_bundle[..], &[fmt_arg, filename]].concat(),
        [&[fmt_arg], &arg_good_bundle[..], &[filename]].concat(),
        [&[fmt_arg], &[filename], &arg_good_bundle[..]].concat(),
    ];

    // test `-X compile`
    #[cfg(feature = "serialization")]
    valid_args.extend([
        [
            &["-X"],
            &["compile"],
            &arg_good_bundle[..],
            &[fmt_arg],
            &[filename],
        ]
        .concat(),
        [
            &["-X"],
            &["compile"],
            &[fmt_arg],
            &arg_good_bundle[..],
            &[filename],
        ]
        .concat(),
        [
            &["-X"],
            &["compile"],
            &[fmt_arg],
            &[filename],
            &arg_good_bundle[..],
        ]
        .concat(),
    ]);

    for args in valid_args {
        let output = run_tectonic(&temppath, &args);
        success_or_panic(&output);
    }
}

/// Test various web bundle overrides for the v2 CLI
#[cfg(feature = "serialization")]
#[test]
fn v2_bundle_overrides() {
    let arg_bad_bundle = ["--bundle", "bad-bundle"];
    let arg_good_bundle = ["--bundle", "test-bundle://"];

    // test `-X command`
    for command in ["new", "init"] {
        // test with a bad bundle
        let tempdir = setup_and_copy_files(&[]);
        let temppath = tempdir.path().to_owned();
        let output = run_tectonic(&temppath, &[&arg_bad_bundle[..], &["-X", command]].concat());
        error_or_panic(&output);

        // test with a good bundle (override)
        let valid_args: Vec<Vec<&str>> = vec![[&["-X", command], &arg_good_bundle[..]].concat()];

        for args in valid_args {
            let tempdir = setup_and_copy_files(&[]);
            let temppath = tempdir.path().to_owned();
            let output = run_tectonic(&temppath, &args);
            success_or_panic(&output);
        }
    }

    // test `-X build`
    let (_tempdir, temppath) = setup_v2();

    // `--bundle` is ignored
    let output = run_tectonic(
        &temppath,
        &[&["-X"], &["build"], &arg_bad_bundle[..]].concat(),
    );
    success_or_panic(&output);
}

#[cfg(feature = "serialization")]
#[test]
fn v2_build_basic() {
    let (_tempdir, temppath) = setup_v2();
    let output = run_tectonic(&temppath, &["-X", "build"]);
    success_or_panic(&output);
}

#[test]
#[cfg(all(feature = "serialization", not(windows)))] // `echo` may not be available
fn v2_build_open() {
    let (_tempdir, temppath) = setup_v2();
    let output = run_tectonic(&temppath, &["-X", "build", "--open"]);
    success_or_panic(&output);
}

#[cfg(feature = "serialization")]
#[test]
fn v2_build_multiple_outputs() {
    util::set_test_root();

    let tempdir = setup_and_copy_files(&[]);
    let mut temppath = tempdir.path().to_owned();
    let output = run_tectonic(&temppath, &["-X", "new", "doc"]);
    success_or_panic(&output);

    temppath.push("doc");

    // To run a build in our test setup, we can only use plain TeX. So, jankily
    // change the format ...

    {
        let mut toml_path = temppath.clone();
        toml_path.push("Tectonic.toml");
        let mut file = OpenOptions::new().append(true).open(toml_path).unwrap();
        writeln!(
            file,
            "tex_format = 'plain'

            [[output]]
            name = 'alt'
            type = 'pdf'
            tex_format = 'plain'
            preamble = '_preamble_alt.tex'
            index = 'index_alt.tex'
            postamble = '_postamble_alt.tex'
            "
        )
        .unwrap();
    }

    // ... and write some files that are plain TeX.

    {
        let mut path = temppath.clone();
        path.push("src");

        {
            path.push("_preamble.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file).unwrap();
            path.pop();
        }
        {
            path.push("_preamble_alt.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file).unwrap();
            path.pop();
        }

        {
            path.push("index_alt.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file, "Hello, alt!").unwrap();
            path.pop();
        }

        {
            path.push("_postamble.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file, "\\end").unwrap();
            path.pop();
        }
        {
            path.push("_postamble_alt.tex");
            let mut file = File::create(&path).unwrap();
            writeln!(file, "\\end").unwrap();
            path.pop();
        }
    }

    // Now we can build.

    let output = run_tectonic(&temppath, &["-X", "build"]);
    success_or_panic(&output);
}

#[test]
#[cfg(feature = "serialization")]
fn v2_dump_basic() {
    let (_tempdir, temppath) = setup_v2();
    let output = run_tectonic(&temppath, &["-X", "dump", "default.log"]);
    success_or_panic(&output);

    let t = std::str::from_utf8(&output.stdout[..]).unwrap();
    let mut saw_it = false;

    for line in t.lines() {
        if line.contains("(default (_preamble.tex) (index.tex) (_postamble.tex [1] ) )") {
            saw_it = true;
            break;
        }
    }

    assert!(saw_it);
}

#[test]
#[cfg(feature = "serialization")]
fn v2_dump_suffix() {
    let (_tempdir, mut temppath) = setup_v2();

    temppath.push("src");
    temppath.push("index.tex");

    {
        let mut file = File::create(&temppath).unwrap();
        #[allow(clippy::write_literal)]
        writeln!(
            file,
            "{}", // <= works around {} fussiness in Rust format strings
            r"\newwrite\w
\immediate\openout\w=first.demo\relax
\immediate\write\w{content-un}
\immediate\closeout\w
\immediate\openout\w=second.demo\relax
\immediate\write\w{content-deux}
\immediate\closeout\w
"
        )
        .unwrap();
    }

    temppath.pop();
    temppath.pop();

    let output = run_tectonic(&temppath, &["-X", "dump", "-s", "demo"]);
    success_or_panic(&output);

    let t = std::str::from_utf8(&output.stdout[..]).unwrap();
    let mut saw_first = false;
    let mut saw_second = false;

    for line in t.lines() {
        if line.contains("content-un") {
            saw_first = true;
        }

        if line.contains("content-deux") {
            saw_second = true;
        }
    }

    assert!(saw_first && saw_second);
}

/// Checks that shell completions are correctly generated
#[cfg(feature = "serialization")]
#[test]
fn v2_show_shell_completions() {
    let (_tempdir, temppath) = setup_v2();
    let output = run_tectonic(&temppath, &["-X", "show", "shell-completions", "zsh"]);
    success_or_panic(&output);

    if !String::from_utf8_lossy(&output.stdout).contains("compdef _nextonic nextonic") {
        panic!("shell completions generation failed.")
    }
}

const SHELL_ESCAPE_TEST_DOC: &str = r"\immediate\write18{mkdir shellwork}
\immediate\write18{echo 123 >shellwork/persist}
\ifnum123=\input{shellwork/persist}
a
\else
\ohnotheshellescapedidntwork
\fi
\bye
";

/// Test that shell escape actually runs the commands
#[test]
fn shell_escape() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);

    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "-Zshell-escape"],
        SHELL_ESCAPE_TEST_DOC,
    );
    success_or_panic(&output);
}

/// Initial revisions with shell-escape ignored any value specified.
/// Rather than allow this to toggle shell-escape which won't work with old installs.
/// Test that shell-escape=false gives an error.
#[test]
fn shell_escape_arg_err() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);

    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "-Zshell-escape=false"],
        SHELL_ESCAPE_TEST_DOC,
    );
    error_or_panic(&output);
}

/// Test that shell-escape can be killed by command-line-option
#[test]
fn shell_escape_cli_override() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);

    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "--untrusted", "-", "-Zshell-escape"],
        SHELL_ESCAPE_TEST_DOC,
    );
    error_or_panic(&output);
}

/// Test that shell-escape can be killed by environment variable
#[test]
fn shell_escape_env_override() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&[]);

    // Note that we intentionally set the variable to 0 below -- it takes it
    // effect if it has ANY value, not just a "truthy" one.

    let mut command = prep_tectonic(tempdir.path(), &[&fmt_arg, "-", "-Zshell-escape"]);
    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env("TECTONIC_UNTRUSTED_MODE", "0");

    println!("running {command:?}");
    let mut child = command.spawn().expect("tectonic failed to start");
    write!(child.stdin.as_mut().unwrap(), "{SHELL_ESCAPE_TEST_DOC}")
        .expect("failed to send data to tectonic subprocess");

    let output = child
        .wait_with_output()
        .expect("failed to wait on tectonic subprocess");

    error_or_panic(&output);
}

/// Test that include paths work
#[test]
fn extra_search_paths() {
    let fmt_arg = get_plain_format_arg();
    let tempdir = setup_and_copy_files(&["subdirectory/content/1.tex"]);

    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[&fmt_arg, "-", "-Zsearch-path=subdirectory/content"],
        "\\input 1.tex\n\\bye",
    );
    success_or_panic(&output);

    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[
            &fmt_arg,
            "-",
            "--hide=subdirectory/content/1.tex",
            "-Zsearch-path=subdirectory/content",
        ],
        "\\input 1.tex\n\\bye",
    );
    error_or_panic(&output);

    let output = run_tectonic_with_stdin(
        tempdir.path(),
        &[
            &fmt_arg,
            "-",
            "-Zsearch-path=subdirectory/content",
            "--untrusted",
        ],
        "\\input 1.tex\n\\bye",
    );
    error_or_panic(&output);
}

/// Ensures that watch command succeeds, and when a file is changed while running it rebuilds
/// periodically
#[cfg(all(feature = "serialization", not(target_arch = "mips")))]
#[test]
fn v2_watch_succeeds() {
    let (_tempdir, temppath) = setup_v2();

    // Timeout the test after 5 minutes - we should definitely run twice in that range
    let max_time = Duration::from_secs(60 * 5);
    let path = temppath.clone();

    // Make sure `default.pdf` already exists - just makes the test easier to implement
    let output = run_tectonic(&temppath, &["-X", "build"]);
    success_or_panic(&output);

    let thread = thread::spawn(move || {
        // Give the process time to start up. Tried a channel, doesn't really work, so we just do
        // a best-effort 'sleep for long enough it should have started'.
        thread::sleep(Duration::from_secs(5));

        let input = path.join("src/index.tex");
        let output = path.join("build/default/default.pdf");
        let start = Instant::now();
        let mut start_mod = None;
        let mut modified = 0;
        while Instant::now() - start < max_time {
            if modified >= 3 {
                break;
            }

            {
                let mut file = File::create(&input).unwrap();
                writeln!(file, "New Text {modified}").unwrap();
            }

            let new_mod = output.metadata().and_then(|meta| meta.modified()).unwrap();
            if start_mod.is_none_or(|start_mod| new_mod > start_mod) {
                start_mod = Some(new_mod);
                modified += 1;
            }

            thread::sleep(Duration::from_secs(5));
        }
    });

    let output = run_tectonic_until(&temppath, &["-X", "watch"], || thread.is_finished());
    // TODO: Make timeout kill child in a way that terminates it gracefully, such as ctrl-c, not SIGKILL
    // success_or_panic(&output);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("-- stdout --\n{stdout}\n-- end stdout --");
    println!("-- stderr --\n{stderr}\n-- end stderr --");

    thread.join().unwrap();

    assert!(stdout.matches("Running TeX").count() >= 2);
}
