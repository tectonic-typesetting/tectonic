// src/engines/tex.rs -- Rustic interface to the core TeX engine.
// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

use crate::errors::DefinitelySame;

pub use tectonic_engine_xetex::{TexEngine, TexResult};

// Sigh, have to do this manually because of the Result/PartialEq conflict in errors.rs
impl DefinitelySame for TexResult {
    fn definitely_same(&self, other: &Self) -> bool {
        self == other
    }
}
