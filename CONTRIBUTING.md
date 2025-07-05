# Contributing to Tectonic

Thanks for your interest in contributing to Tectonic!

There are a variety of ways to contribute to the project. This document is
concerned with things that you do on GitHub: submitting code and managing bug
reports. Please see
[the corresponding section of the main Tectonic website](https://tectonic-typesetting.github.io/contribute.html)
for a bigger-picture overview.

## Code of Conduct

The only restriction for contributions is that you must abide by Tectonic’s
[Code of Conduct](./CODE_OF_CONDUCT.md). In summary, it says that you should
not act like a jerk. Good-faith efforts to follow the letter and spirit of the
Code of Conduct are *required* of all Tectonic community members.

## Pull Requests

Pull requests are very welcome! If you are pondering a substantial change,
please
[open an issue](https://github.com/tectonic-typesetting/tectonic/issues/new)
before starting work so that the design can be discussed with the development
team.

The [CI](https://en.wikipedia.org/wiki/Continuous_integration) system requires
that all submitted Rust code be formatted according to the
[rustfmt](https://github.com/rust-lang/rustfmt#readme) tool. We *strongly*
recommend that you achieve this by configuring your text editor to apply
`rustfmt` formatting every time you save a file,
[as per these instructions](https://github.com/rust-lang/rustfmt#running-rustfmt-from-your-editor).
The Git history stays easier to read this way since we avoid making commits
that simply reformat existing code.

## Legalities

New Tectonic code is licensed under the MIT License, and we assign its copyright
to "the Tectonic Project". Any code that you submit to Tectonic will be assumed
to be made available under these terms unless explicitly stated otherwise.
