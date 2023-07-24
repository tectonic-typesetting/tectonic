use std::path::PathBuf;
use std::str::from_utf8;

// Use `pub` to work around "dead code" warnings;
// https://github.com/rust-lang/rust/issues/46379#issuecomment-548787629
pub mod executable_util;
use executable_util::{error_or_panic, run_tectonic, success_or_panic};

// BEGIN TESTS

#[cfg(feature = "serialization")]
#[test]
fn v2_help_succeeds() {
    let output = run_tectonic(&PathBuf::from("."), &["-X", "--help"]);
    success_or_panic(&output);
}

#[cfg(feature = "serialization")]
#[test]
fn v2_help_on_no_subcmd() {
    let output = run_tectonic(&PathBuf::from("."), &["-X"]);
    error_or_panic(&output);
}

#[cfg(feature = "serialization")]
#[test]
fn v2_helps_equal() {
    let output_long = run_tectonic(&PathBuf::from("."), &["-X", "--help"]);

    let output_short = run_tectonic(&PathBuf::from("."), &["-X", "-h"]);
    assert_eq!(&output_long, &output_short);

    let output_subcmd = run_tectonic(&PathBuf::from("."), &["-X", "help"]);
    assert_eq!(&output_long, &output_subcmd);

    let output_no_subcmd = run_tectonic(&PathBuf::from("."), &["-X"]);
    assert_eq!(
        &[&output_long.stdout, &output_long.stderr],
        &[&output_no_subcmd.stderr, &output_no_subcmd.stdout]
    );
}

/// Test that flags are ordered alphabetically according to their long name.
#[cfg(feature = "serialization")]
#[test]
fn v2_help_flags_ordered() {
    test_v2_help_section("FLAGS", parse_v2_help_arg);
}

/// Test that options are ordered alphabetically according to their long name.
#[cfg(feature = "serialization")]
#[test]
fn v2_help_options_ordered() {
    test_v2_help_section("OPTIONS", parse_v2_help_arg);
}

/// Test that subcommands are ordered alphabetically.
#[cfg(feature = "serialization")]
#[test]
fn v2_help_subcmds_ordered() {
    test_v2_help_section("SUBCOMMANDS", parse_v2_help_subcmd);
}

// END TESTS

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

fn test_v2_help_section_lines<'a>(
    lines: impl IntoIterator<Item = &'a str>,
    parse: impl Fn(&str) -> &str,
) {
    // Parse the argument or subcommand name from each line, and compare it to
    // the previous one to ensure they are ordered alphabetically
    lines.into_iter().fold(None, |prev_name, line| {
        let line = line.get(HELP_INDENT..).unwrap_or_else(|| {
            panic!("line should be indented by at least {} spaces", HELP_INDENT)
        });

        match line.chars().next() {
            None => panic!("help section should not have empty lines"),
            Some(' ') => {
                // This is a continuation of the previous line, so skip it
                return prev_name;
            }
            Some(_) => {}
        }

        // In ASCII, uppercase letters preceed lowercase letters, so
        // flags like `-Z` would preceed `-a`. To avoid this, we compare
        // in all lowercase.
        let name = parse(line).to_lowercase();
        if let Some(prev_name) = prev_name {
            assert!(
                name >= prev_name,
                "args in help message should be ordered alphabetically"
            );
        }
        Some(name)
    });
}

fn test_v2_help_section(name: &str, parse: impl Fn(&str) -> &str) {
    let output = run_tectonic(&PathBuf::from("."), &["-X", "--help"]);
    let output = from_utf8(&output.stdout).expect("help message should be valid UTF-8");

    // Start after the section heading
    let lines_onward = output
        .split_once(&format!("{}:\n", name))
        .unwrap_or_else(|| panic!("help message should have a \"{}\" section", name))
        .1;

    // Stop at the end of the section
    let lines = lines_onward
        .split_once("\n\n")
        .map_or(lines_onward, |(lines, _)| lines)
        .split_terminator('\n');

    test_v2_help_section_lines(lines, parse);
}

/// Parse the argument name from an unindented line in a help message.
///
/// This is the long version of the name if it exists, or the short version
/// otherwise.
///
/// For example, `-X, --example    Description...` is parsed as `example`, while
/// `-X    Description...` is parsed as `X`. This is true regardless of whether
/// there is a description.
fn parse_v2_help_arg(line: &str) -> &str {
    // Start at the argument name
    let name_onward = line
        .split_once("--")
        .or_else(|| line.split_once('-'))
        .expect("line should begin with an argument")
        .1;

    // Stop at the end of the argument name
    let name = name_onward
        .split_once(' ')
        .map_or(name_onward, |(name, _)| name);

    name
}

/// Parse the subcommand name from an unindented line in a help message.
///
/// For example, `example    Description...` is parsed as `example` (regardless
/// of whether there is a description).
fn parse_v2_help_subcmd(line: &str) -> &str {
    line.split_once(' ').map_or(line, |(name, _)| name)
}
