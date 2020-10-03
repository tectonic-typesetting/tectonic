// src/bin/tectonic.rs -- Command-line driver for the Tectonic engine.
// Copyright 2020 the Tectonic Project
// Licensed under the MIT License.

//! Unstable options for the Tectonic engine.
//!
//! This is similar to the -Z options on rustc - they're unstable options that are not guaranteed
//! to be reliable or very polished. In particular, many of these prevent the build from being
//! reproducible.

use crate::errors::Result;
use std::default::Default;

#[derive(Debug)]
pub struct UnstableOptions {
    /// The output paper size to use, instead of "letter"
    pub paper_size: Option<String>,
    /// Whether to enable shell escape
    pub shell_escape: bool,
}

impl Default for UnstableOptions {
    fn default() -> Self {
        Self {
            paper_size: None,
            shell_escape: false,
        }
    }
}

impl UnstableOptions {
    pub fn parse_single(&mut self, opt: &str) -> Result<()> {
        let mut splitted = opt.splitn(2, '=');
        let name = splitted.next().unwrap();
        let arg = splitted.next().ok_or("option requres an argument");

        match name {
            "paper-size" => self.paper_size = Some(arg?.to_string()),
            "shell-escape" => self.shell_escape = true,
            "no-shell-escape" => self.shell_escape = false,
            _ => return Err("invalid option".into()),
        }

        Ok(())
    }

    pub fn from_strings<'a, I>(options: I) -> Result<Self>
    where
        I: Iterator<Item = &'a str>,
    {
        let mut config = Self::default();
        for arg in options {
            config.parse_single(arg)?;
        }

        Ok(config)
    }
}
