use rstest::rstest;
use rstest_reuse::{self, apply, template};
use std::path::PathBuf;
use std::str::from_utf8;

// Use `pub` to work around "dead code" warnings;
// https://github.com/rust-lang/rust/issues/46379#issuecomment-548787629
pub mod executable_util;
use executable_util::{error_or_panic, run_tectonic, success_or_panic};

// BEGIN TESTS

#[cfg(feature = "serialization")]
#[apply(all_commands)]
fn help_succeeds(#[case] args: &[&str]) {
    let args = [args, &["--help"]].concat();
    let output = run_tectonic(&PathBuf::from("."), &args);
    success_or_panic(&output);
}

#[cfg(feature = "serialization")]
#[apply(cmds_with_subcmds)]
fn help_on_no_subcmd(#[case] args: &[&str]) {
    let output = run_tectonic(&PathBuf::from("."), args);
    error_or_panic(&output);
}

#[cfg(feature = "serialization")]
#[apply(all_commands)]
fn helps_equal(#[case] args: &[&str]) {
    let args_long = [args, &["--help"]].concat();
    let output_long = run_tectonic(&PathBuf::from("."), &args_long);

    let args_short = [args, &["-h"]].concat();
    let output_short = run_tectonic(&PathBuf::from("."), &args_short);
    assert_eq!(&output_long, &output_short);

    let has_subcmd = from_utf8(&output_long.stdout)
        .ok()
        .and_then(|output| output.find("SUBCOMMANDS:\n"))
        .is_some();
    if has_subcmd {
        let args_subcmd = [args, &["help"]].concat();
        let output_subcmd = run_tectonic(&PathBuf::from("."), &args_subcmd);
        assert_eq!(&output_long, &output_subcmd);

        let output_no_subcmd = run_tectonic(&PathBuf::from("."), args);
        assert_eq!(
            &[&output_long.stdout, &output_long.stderr],
            &[&output_no_subcmd.stderr, &output_no_subcmd.stdout]
        );
    }
}

/// Test that flags are ordered lexicographically according to their long name.
#[cfg(feature = "serialization")]
#[apply(cmds_with_flags)]
fn help_flags_ordered(#[case] args: &[&str]) {
    test_help_section(args, "FLAGS", parse_help_flag);
}

/// Test that options are ordered lexicographically according to their argument.
#[cfg(feature = "serialization")]
#[apply(cmds_with_options)]
fn help_options_ordered(#[case] args: &[&str]) {
    test_help_section(args, "OPTIONS", parse_help_option);
}

/// Test that subcommands are ordered lexicographically.
#[cfg(feature = "serialization")]
#[apply(cmds_with_subcmds)]
fn help_subcmds_ordered(#[case] args: &[&str]) {
    test_help_section(args, "SUBCOMMANDS", parse_help_subcmd);
}

// END TESTS

#[template]
#[rstest]
#[case::v1_main(&[])]
#[case::v2_main(&["-X"])]
#[case::v2_build(&["-X", "build"])]
#[case::v2_bundle_main(&["-X", "bundle"])]
#[case::v2_bundle_cat(&["-X", "bundle", "cat"])]
#[case::v2_bundle_search(&["-X", "bundle", "search"])]
#[case::v2_compile(&["-X", "compile"])]
#[case::v2_dump(&["-X", "dump"])]
#[case::v2_init(&["-X", "init"])]
#[case::v2_new(&["-X", "new"])]
#[case::v2_show_main(&["-X", "show"])]
#[case::v2_show_user_cache_dir(&["-X", "show", "user-cache-dir"])]
#[case::v2_watch(&["-X", "watch"])]
fn all_commands(#[case] args: &[&str]) {}

#[template]
#[apply(all_commands)]
fn cmds_with_flags(#[case] args: &[&str]) {}

#[template]
#[rstest]
#[case::v1_main(&[])]
#[case::v2_main(&["-X"])]
#[case::v2_build(&["-X", "build"])]
#[case::v2_compile(&["-X", "compile"])]
#[case::v2_dump(&["-X", "dump"])]
#[case::v2_watch(&["-X", "watch"])]
fn cmds_with_options(#[case] args: &[&str]) {}

#[template]
#[rstest]
#[case::v2_main(&["-X"])]
#[case::v2_bundle_main(&["-X", "bundle"])]
#[case::v2_show_main(&["-X", "show"])]
fn cmds_with_subcmds(#[case] args: &[&str]) {}

/// The number of spaces leading up to a flag, option, or subcommand.
///
/// Without this, it becomes annoying to parse the help message for argument
/// names. For example, consider part of a help message:
///
/// ```text
/// FLAGS:
///     --example0    Long description that gets wrapped
///                   --oops, we used a double hyphen
///
/// SUBCOMMANDS:
///     example1    Another long description that gets
///                 wrapped
/// ```
///
/// If we naively trim leading spaces, then we get:
///
/// ```text
/// FLAGS:
/// --example0    Long description that gets wrapped
/// --oops, we used a double hyphen
///
/// SUBCOMMANDS:
/// example1    Another long description that gets
/// wrapped
/// ```
///
/// How do we know `--oops` is not a flag and `wrapped` is not a subcommand?
///
/// It is easier if we take the indent into account by only matching text that
/// comes right after it.
const HELP_INDENT: usize = 4;

fn test_help_section_lines<'a>(
    lines: impl IntoIterator<Item = &'a str>,
    parse: impl Fn(&str) -> &str,
) {
    // Parse the argument or subcommand name from each line, and compare it to
    // the previous one to ensure they are ordered lexicographically
    lines.into_iter().fold("", |prev_name, line| {
        let line = line.get(HELP_INDENT..).unwrap_or_else(|| {
            panic!("line should be indented by at least {} spaces", HELP_INDENT)
        });

        let name = parse(line);

        let prev_name_lower = prev_name.chars().next().map(char::is_lowercase);
        let name_lower = name.chars().next().map(char::is_lowercase);

        let ordered = match (prev_name_lower, name_lower) {
            (_, None) => return prev_name,
            (None, _) => return name,
            (Some(true), Some(true)) | (Some(false), Some(false)) => prev_name <= name,
            (Some(true), Some(false)) => prev_name <= name.to_lowercase().as_str(),
            // We do not want uppercase flags to preceed lowercase ones, even
            // though that is how they are ordered in ASCII.
            (Some(false), Some(true)) => false,
        };
        assert!(
            ordered,
            "\"{}\" preceeds \"{}\", but lines should be ordered lexicographically",
            prev_name, name
        );
        name
    });
}

fn test_help_section(args: &[&str], name: &str, parse: impl Fn(&str) -> &str) {
    let args = [args, &["--help"]].concat();
    let output = run_tectonic(&PathBuf::from("."), &args);
    let output = from_utf8(&output.stdout).expect("message should be valid UTF-8");

    // Start after the section heading
    let lines_onward = output
        .split_once(&format!("{}:\n", name))
        .unwrap_or_else(|| panic!("message should have a \"{}\" section", name))
        .1;

    // Stop at the end of the section
    let lines = lines_onward
        .split_once("\n\n")
        .map_or(lines_onward, |(lines, _)| lines)
        .split_terminator('\n');

    test_help_section_lines(lines, parse);
}

/// Parse the flag name from an unindented line in a help message.
///
/// This is the long version of the name if it exists, or the short version
/// otherwise.
///
/// For example, `-X, --example    Description...` is parsed as `example`, while
/// `-X    Description...` is parsed as `X`. This is true regardless of whether
/// there is a description.
fn parse_help_flag(line: &str) -> &str {
    // Start at the flag name
    let name_onward = match line.get(HELP_INDENT..HELP_INDENT + 2) {
        Some("--") => &line[HELP_INDENT + 2..],
        Some("  ") => &line[1..],
        None => line.get(1..).expect("line should start with a flag"),
        Some(s) => panic!("line should not contain text at this position: \"{}\"", s),
    };

    // Stop at the end of the flag name
    let name = name_onward
        .split_once(' ')
        .map_or(name_onward, |(name, _)| name);

    name
}

/// Parse the option argument from an unindented line in a help message.
///
/// For example, `-X, --example <arg>    Description...` is parsed as `arg`,
/// regardless of the presence of the short option name, long option name, and
/// description.
fn parse_help_option(line: &str) -> &str {
    if line.starts_with(' ') {
        return "";
    }

    // Start at the argument name
    let name_onward = line
        .split_once('<')
        .expect("option should have an argument")
        .1;

    // Stop at the end of the argument name
    let name = name_onward
        .split_once('>')
        .expect("argument should be enclosed in angle brackets")
        .0;

    name
}

/// Parse the subcommand name from an unindented line in a help message.
///
/// For example, `example    Description...` is parsed as `example` (regardless
/// of whether there is a description).
fn parse_help_subcmd(line: &str) -> &str {
    line.split_once(' ').map_or(line, |(name, _)| name)
}
