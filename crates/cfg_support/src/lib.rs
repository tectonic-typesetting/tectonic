// Copyright 2019-2020 the Tectonic Project
// Licensed under the MIT License.

//! This support crate helps deal with `CARGO_CFG_TARGET_*` variables. When
//! cross-compiling with a `build.rs` script, these variables must be used
//! instead of constructs such as `cfg!(target_arch = ...)` because the
//! build.rs compilation targets the build host architecture, not the final
//! target architecture.
//!
//! For more information, see the documentation on:
//!
//! * [cargo environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
//! * [conditional compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)

// Debugging help (requires nightly):
//#![feature(trace_macros)]
//trace_macros!(true);

use lazy_static::lazy_static;

lazy_static! {
    /// Singleton initialized instance of the compilation target info
    pub static ref TARGET_CONFIG: TargetConfiguration = TargetConfiguration::default();
}

#[derive(Clone, Debug)]
/// Information about the compilation target.
///
/// These parameters are derived from the `CARGO_TARGET_CFG_*` environment
/// variables, which must be used to obtain correct results when
/// cross-compiling a `build.rs` script. The configuration values are
/// uppercased when they're loaded, to allow for case-insensitive comparisons
/// later.
pub struct TargetConfiguration {
    /// Equivalent to `target_arch`
    pub arch: String,
    /// Equivalent to `target_feature`
    pub feature: String,
    /// Equivalent to `target_os`
    pub os: String,
    /// Equivalent to `target_family`
    pub family: String,
    /// Equivalent to `target_env`
    pub env: String,
    /// Equivalent to `target_endian`
    pub endian: String,
    /// Equivalent to `target_pointer_width`
    pub pointer_width: String,
    /// Equivalent to `target_vendor`
    pub vendor: String,
}

impl Default for TargetConfiguration {
    /// Creates a TargetConfiguration from the `CARGO_CFG_TARGET_*`
    /// [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html)
    fn default() -> Self {
        fn getenv(var: &'static str) -> String {
            std::env::var(var)
                .unwrap_or_else(|_| String::new())
                .to_uppercase()
        }

        TargetConfiguration {
            arch: getenv("CARGO_CFG_TARGET_ARCH"),
            feature: getenv("CARGO_CFG_TARGET_FEATURE"),
            os: getenv("CARGO_CFG_TARGET_OS"),
            family: getenv("CARGO_CFG_TARGET_FAMILY"),
            env: getenv("CARGO_CFG_TARGET_ENV"),
            endian: getenv("CARGO_CFG_TARGET_ENDIAN"),
            pointer_width: getenv("CARGO_CFG_TARGET_POINTER_WIDTH"),
            vendor: getenv("CARGO_CFG_TARGET_VENDOR"),
        }
    }
}

impl TargetConfiguration {
    /// Test whether the target architecture exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_arch(&self, arch: &str) -> bool {
        self.arch == arch.to_uppercase()
    }

    /// Test whether the target OS exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_os(&self, os: &str) -> bool {
        self.os == os.to_uppercase()
    }

    /// Test whether the target family exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_family(&self, family: &str) -> bool {
        self.family == family.to_uppercase()
    }

    /// Test whether the target "environment" exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_env(&self, env: &str) -> bool {
        self.env == env.to_uppercase()
    }

    /// Test whether the target endianness exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_endian(&self, endian: &str) -> bool {
        self.endian == endian.to_uppercase()
    }

    /// Test whether the target pointer width exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_pointer_width(&self, pointer_width: &str) -> bool {
        self.pointer_width == pointer_width.to_uppercase()
    }

    /// Test whether the target vendor exactly matches the argument, in
    /// case-insensitive fashion.
    pub fn target_vendor(&self, vendor: &str) -> bool {
        self.vendor == vendor.to_uppercase()
    }
}

/// Test for characteristics of the target machine.
///
/// Unlike the standard `cfg!` macro, this macro will give correct results
/// when cross-compiling in a build.rs script. It attempts, but is not
/// guaranteed, to emulate the syntax of the `cfg!` macro. Note, however,
/// that the result of the macro must be evaluated at runtime, not compile-time.
///
/// Supported syntaxes:
///
/// ```notest
/// target_cfg!(target_os = "macos");
/// target_cfg!(not(target_os = "macos"));
/// target_cfg!(any(target_os = "macos", target_endian = "big"));
/// target_cfg!(all(target_os = "macos", target_endian = "big"));
/// target_cfg!(all(target_os = "macos", not(target_endian = "big")));
/// ```
// Here we go with some exciting macro fun!
//
// Since each individual test can be evaluated to a boolean on-the-spot, the
// macro expands out to a big boolean logical expression. Fundamentally, it's
// not too hard to allow complex syntax because the macro can recurse:
//
// ```
// target_cfg!(not(whatever)) => !(target_cfg!(whatever))
// target_cfg!(any(c1, c2)) => target_cfg!(c1) || target_cfg!(c2)
// ```
//
// The core implementation challenge here is that we need to parse
// comma-separated lists where each term might contain all sorts of unexpected
// content. Within the confines of the macro_rules! formalism, this means that
// we need to scan through such comma-separated lists and group their tokens
// before actually evaluating them.
//
// Some key points to remember about how this all works:
//
// 1. A "token tree" type is either a single token or a series of tokens
//    delimited by balanced delimiters such as ({[]}). As such, in order to
//    match an arbitrary token sequence, you need to use repetition
//    expressions of the form `$($toks:tt)+`.
// 2. The macro evaluator looks at rules in order and cannot backtrack. That
//    is, if it is looking at a rule and has matched the first 5 tokens but
//    the 6th disagrees, there must be a subsequent rule that also matches
//    those first 5 tokens.
// 3. Given the above, we use the standard trick of having different macro
//    "modes" prefixed with an expression like `@emit`. They are essentially
//    different sub-macros but this trick allows us to get everything done
//    with one named macro_rules! export.
// 4. Also due to the above, the logical flow of the macro generally goes from
//    bottom to top, so that's probably the best way to read the code.
//
// Some links for reference:
//
// - https://users.rust-lang.org/t/top-down-macro-parsing-or-higher-order-macros/8879
// - https://danielkeep.github.io/tlborm/book/pat-incremental-tt-munchers.html
#[macro_export]
macro_rules! target_cfg {
    // "@emit" rules are used for comma-separated lists that have had their
    // tokens grouped. The general pattern is: `target_cfg!(@emit $operation
    // {clause1..} {clause2..} {clause3..})`.

    // Emitting `any(clause1,clause2,...)`: convert to `target_cfg!(clause1) && target_cfg!(clause2) && ...`
    (
        @emit
        all
        $({$($grouped:tt)+})+
    ) => {
        ($(
            target_cfg!($($grouped)+)
        )&&+)
    };

    // Likewise for `all(clause1,clause2,...)`.
    (
        @emit
        any
        $({$($grouped:tt)+})+
    ) => {
        ($(
            target_cfg!($($grouped)+)
        )||+)
    };

    // "@clause" rules are used to parse the comma-separated lists. They munch
    // their inputs token-by-token and finally invoke an "@emit" rule when the
    // list is all grouped. The general pattern for recording the parser state
    // is:
    //
    // ```
    // target_cfg!(
    //    @clause $operation
    //    [{grouped-clause-1} {grouped-clause-2...}]
    //    [not-yet-parsed-tokens...]
    //    current-clause-tokens...
    // )
    // ```

    // This rule must come first in this section. It fires when the next token
    // to parse is a comma. When this happens, we take the tokens in the
    // current clause and add them to the list of grouped clauses, adding
    // delimeters so that the grouping can be easily extracted again in the
    // emission stage.
    (
        @clause
        $op:ident
        [$({$($grouped:tt)+})*]
        [, $($rest:tt)*]
        $($current:tt)+
    ) => {
        target_cfg!(@clause $op [
            $(
                {$($grouped)+}
            )*
            {$($current)+}
        ] [
            $($rest)*
        ])
    };

    // This rule comes next. It fires when the next un-parsed token is *not* a
    // comma. In this case, we add that token to the list of tokens in the
    // current clause, then move on to the next one.
    (
        @clause
        $op:ident
        [$({$($grouped:tt)+})*]
        [$tok:tt $($rest:tt)*]
        $($current:tt)*
    ) => {
        target_cfg!(@clause $op [
            $(
                {$($grouped)+}
            )*
        ] [
            $($rest)*
        ] $($current)* $tok)
    };

    // This rule fires when there are no more tokens to parse in this list. We
    // finish off the "current" token group, then delegate to the emission
    // rule.
    (
        @clause
        $op:ident
        [$({$($grouped:tt)+})*]
        []
        $($current:tt)+
    ) => {
        target_cfg!(@emit $op
            $(
                {$($grouped)+}
            )*
            {$($current)+}
        )
    };

    // Finally, these are the "toplevel" syntaxes for specific tests that can
    // be performed. Any construction not prefixed with one of the magic
    // tokens must match one of these.

    // `all(clause1, clause2...)` : we must parse this comma-separated list and
    // partner with `@emit all` to output a bunch of && terms.
    (
        all($($tokens:tt)+)
    ) => {
        target_cfg!(@clause all [] [$($tokens)+])
    };

    // Likewise for `any(clause1, clause2...)`
    (
        any($($tokens:tt)+)
    ) => {
        target_cfg!(@clause any [] [$($tokens)+])
    };

    // `not(clause)`: compute the inner clause, then just negate it.
    (
        not($($tokens:tt)+)
    ) => {
        !(target_cfg!($($tokens)+))
    };

    // `param = value`: test for equality.
    (
        $e:tt = $v:expr
    ) => {
        $crate::TARGET_CONFIG.$e($v)
    };
}

#[cfg(test)]
mod tests {
    /// Set up the environment variables for testing. We intentionally choose
    /// values that don't occur in the real world, except for parameters that
    /// have heavily constrained options, to avoid accidentally passing
    /// if/when running the test suite on familiar hardware.
    fn setup_test_env() {
        std::env::set_var("CARGO_CFG_TARGET_ARCH", "testarch");
        std::env::set_var("CARGO_CFG_TARGET_FEATURE", "testfeat1,testfeat2");
        std::env::set_var("CARGO_CFG_TARGET_OS", "testos");
        std::env::set_var("CARGO_CFG_TARGET_FAMILY", "testfamily");
        std::env::set_var("CARGO_CFG_TARGET_ENV", "testenv");
        std::env::set_var("CARGO_CFG_TARGET_ENDIAN", "little");
        std::env::set_var("CARGO_CFG_TARGET_POINTER_WIDTH", "32");
        std::env::set_var("CARGO_CFG_TARGET_VENDOR", "testvendor");
    }

    /// No recursion. Check all of the supported tests.
    #[test]
    fn test_level0() {
        setup_test_env();

        assert!(target_cfg!(target_arch = "testarch"));
        assert!(!target_cfg!(target_arch = "wrong"));

        assert!(target_cfg!(target_os = "testos"));
        assert!(target_cfg!(target_family = "testfamily"));
        assert!(target_cfg!(target_env = "testenv"));
        assert!(target_cfg!(target_endian = "little"));
        assert!(target_cfg!(target_pointer_width = "32"));
        assert!(target_cfg!(target_vendor = "testvendor"));
    }

    /// Basic recursion.
    #[test]
    fn test_level1() {
        setup_test_env();

        assert!(target_cfg!(not(target_arch = "wrong")));
        assert!(!target_cfg!(not(target_arch = "testarch")));

        assert!(target_cfg!(all(target_arch = "testarch")));
        assert!(!target_cfg!(all(target_arch = "wrong")));
        assert!(target_cfg!(all(
            target_arch = "testarch",
            target_os = "testos"
        )));
        assert!(!target_cfg!(all(
            target_arch = "testarch",
            target_os = "wrong"
        )));

        assert!(target_cfg!(any(target_arch = "testarch")));
        assert!(!target_cfg!(any(target_arch = "wrong")));
        assert!(target_cfg!(any(
            target_arch = "testarch",
            target_os = "testos"
        )));
        assert!(target_cfg!(any(
            target_arch = "testarch",
            target_os = "wrong"
        )));
        assert!(!target_cfg!(any(
            target_arch = "wrong1",
            target_os = "wrong2"
        )));
    }

    /// Even deeper recursion.
    #[test]
    fn test_level2() {
        setup_test_env();

        assert!(target_cfg!(all(not(target_arch = "wrong"))));
        assert!(!target_cfg!(all(not(target_arch = "testarch"))));

        assert!(target_cfg!(all(
            target_arch = "testarch",
            not(target_os = "wrong")
        )));

        assert!(target_cfg!(all(
            any(target_arch = "testarch", target_os = "wrong"),
            target_env = "testenv"
        )));

        assert!(target_cfg!(all(
            any(target_arch = "testarch", target_os = "wrong"),
            not(target_vendor = "wrong")
        )));
    }
}
