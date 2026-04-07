use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct BundleSpec {
    pub bundle: BundleConfig,
    pub inputs: HashMap<String, BundleInput>,
}

impl BundleSpec {
    /// Make sure this bundle specification is valid
    pub fn validate(&self) -> Result<()> {
        for i in &self.bundle.search_order {
            match i {
                BundleSearchOrder::Input { ref input } => {
                    if !self.inputs.contains_key(input) {
                        bail!("root search order contains unknown input `{input}`");
                    }
                }
                BundleSearchOrder::Plain(_) => {}
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct BundleConfig {
    /// The bundle's name
    pub name: String,

    /// The hash of the resulting ttbv1 bundle
    pub expected_hash: String,

    /// Search paths for this bundle
    pub search_order: Vec<BundleSearchOrder>,

    /// Files to ignore from this input
    pub ignore: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum BundleSearchOrder {
    Plain(String),
    Input { input: String },
}

#[derive(Debug, Deserialize, Clone)]
pub struct BundleInput {
    pub source: BundleInputSource,
    pub ignore: Option<Vec<String>>,
    pub patch_dir: Option<PathBuf>,
    pub search_order: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum BundleInputSource {
    #[serde(rename = "dir")]
    Directory { path: PathBuf },

    #[serde(rename = "tarball")]
    Tarball {
        hash: String,
        path: PathBuf,
        root_dir: Option<PathBuf>,
    },
}
