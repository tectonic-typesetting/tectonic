// Copyright 2020-2023 the Tectonic Project
// Licensed under the MIT License.

//! This file defines the syntax of Tectonic.toml,
//! which is parsed using serde.
//!
//! This module is only used by [`crate::document::Document`]

use crate::document::{BuildTargetType, InputFile, OutputProfile};
use serde::{Deserialize, Serialize, Serializer};
use std::path::PathBuf;

// This file is an exercise in Rust type conversion.
//
// Every stuct or enum that starts with "Toml*" is a
// serializable version of a struct or enum in document.rs.
// We convert between the two with ::from() and .into().

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TomlDocument {
    pub doc: TomlDocSection,

    #[serde(rename = "output")]
    pub outputs: Vec<TomlOutputProfile>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TomlDocSection {
    pub name: String,
    pub bundle: String,
    pub extra_paths: Option<Vec<PathBuf>>,
    pub metadata: Option<toml::Value>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum StringOrInputVec {
    String(TomlInputFile),
    Vec(Vec<TomlInputFile>),
}

// Minor modification on the default serializer.
// Omits array brackets when we only have one input file.
impl Serialize for StringOrInputVec {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::String(s) => s.serialize(serializer),
            Self::Vec(v) => {
                if v.len() == 1 {
                    v[0].serialize(serializer)
                } else {
                    v.serialize(serializer)
                }
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum TomlInputFile {
    Inline { inline: String },
    Path(String),
}

impl From<&InputFile> for TomlInputFile {
    fn from(val: &InputFile) -> TomlInputFile {
        match val {
            InputFile::Inline(inline) => TomlInputFile::Inline {
                inline: inline.clone(),
            },
            InputFile::File(f) => TomlInputFile::Path(f.clone()),
        }
    }
}

impl From<TomlInputFile> for InputFile {
    fn from(val: TomlInputFile) -> InputFile {
        match val {
            TomlInputFile::Inline { inline } => InputFile::Inline(inline),
            TomlInputFile::Path(f) => InputFile::File(f),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TomlOutputProfile {
    pub name: String,
    #[serde(rename = "type")]
    pub target_type: TomlBuildTargetType,
    pub tex_format: Option<String>,
    pub shell_escape: Option<bool>,
    pub shell_escape_cwd: Option<String>,

    // We cannot handle these two input variants with an enum.
    // The ideal solution requires #[serde(flatten)],
    // which is incompatible with deny_unknown_fields.
    // This will have to do for now.
    pub inputs: Option<StringOrInputVec>,

    // Old-fashioned file inputs
    // we might want to deprecate these eventually, or at least provide a warning.
    #[serde(rename = "preamble")]
    pub preamble_file: Option<String>,
    #[serde(rename = "index")]
    pub index_file: Option<String>,
    #[serde(rename = "postamble")]
    pub postamble_file: Option<String>,
}

impl From<&TomlOutputProfile> for OutputProfile {
    fn from(val: &TomlOutputProfile) -> OutputProfile {
        let shell_escape_default = val.shell_escape_cwd.is_some();

        let inputs = {
            if let Some(inputs) = &val.inputs {
                match inputs {
                    StringOrInputVec::String(s) => vec![s.clone().into()],
                    StringOrInputVec::Vec(v) => v.iter().map(|x| x.clone().into()).collect(),
                }
            } else {
                let mut v = Vec::with_capacity(3);
                if let Some(s) = &val.preamble_file {
                    v.push(TomlInputFile::Path(s.to_string()).into())
                }
                if let Some(s) = &val.index_file {
                    v.push(TomlInputFile::Path(s.to_string()).into())
                }
                if let Some(s) = &val.postamble_file {
                    v.push(TomlInputFile::Path(s.to_string()).into())
                }
                v
            }
        };

        OutputProfile {
            name: val.name.clone(),
            target_type: val.target_type.into(),
            tex_format: val
                .tex_format
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or("latex")
                .to_owned(),
            inputs,
            shell_escape: val.shell_escape.unwrap_or(shell_escape_default),
            shell_escape_cwd: val.shell_escape_cwd.clone(),
        }
    }
}

impl From<&OutputProfile> for TomlOutputProfile {
    fn from(rt: &OutputProfile) -> Self {
        let tex_format = if rt.tex_format == "latex" {
            None
        } else {
            Some(rt.tex_format.clone())
        };

        let inputs: StringOrInputVec =
            StringOrInputVec::Vec(rt.inputs.clone().iter().map(TomlInputFile::from).collect());

        let shell_escape = if !rt.shell_escape { None } else { Some(true) };
        let shell_escape_cwd = rt.shell_escape_cwd.clone();

        TomlOutputProfile {
            name: rt.name.clone(),
            target_type: TomlBuildTargetType::from(&rt.target_type),
            tex_format,
            inputs: Some(inputs),
            shell_escape,
            shell_escape_cwd,
            preamble_file: None,
            index_file: None,
            postamble_file: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum TomlBuildTargetType {
    #[serde(rename = "html")]
    Html,

    #[serde(rename = "pdf")]
    Pdf,
}

impl From<TomlBuildTargetType> for BuildTargetType {
    fn from(val: TomlBuildTargetType) -> BuildTargetType {
        match val {
            TomlBuildTargetType::Html => BuildTargetType::Html,
            TomlBuildTargetType::Pdf => BuildTargetType::Pdf,
        }
    }
}

impl From<&BuildTargetType> for TomlBuildTargetType {
    fn from(s: &BuildTargetType) -> Self {
        match s {
            BuildTargetType::Html => TomlBuildTargetType::Html,
            BuildTargetType::Pdf => TomlBuildTargetType::Pdf,
        }
    }
}
