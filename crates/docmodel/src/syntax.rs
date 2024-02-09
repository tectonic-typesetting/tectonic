use crate::document::{
    BuildTargetType, OutputProfile, DEFAULT_INDEX_FILE, DEFAULT_POSTAMBLE_FILE,
    DEFAULT_PREAMBLE_FILE,
};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

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
    pub metadata: Option<toml::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TomlOutputProfile {
    pub name: String,
    #[serde(rename = "type")]
    pub target_type: TomlBuildTargetType,
    pub tex_format: Option<String>,
    #[serde(rename = "preamble")]
    pub preamble_file: Option<String>,
    #[serde(rename = "index")]
    pub index_file: Option<String>,
    #[serde(rename = "postamble")]
    pub postamble_file: Option<String>,
    pub shell_escape: Option<bool>,
    pub shell_escape_cwd: Option<String>,
}

impl TomlOutputProfile {
    pub fn from_runtime(rt: &OutputProfile) -> Self {
        let tex_format = if rt.tex_format == "latex" {
            None
        } else {
            Some(rt.tex_format.clone())
        };

        let preamble_file = if rt.preamble_file == DEFAULT_PREAMBLE_FILE {
            None
        } else {
            Some(rt.preamble_file.clone())
        };

        let index_file = if rt.index_file == DEFAULT_INDEX_FILE {
            None
        } else {
            Some(rt.index_file.clone())
        };

        let postamble_file = if rt.postamble_file == DEFAULT_POSTAMBLE_FILE {
            None
        } else {
            Some(rt.postamble_file.clone())
        };

        let shell_escape = if !rt.shell_escape { None } else { Some(true) };
        let shell_escape_cwd = rt.shell_escape_cwd.clone();

        TomlOutputProfile {
            name: rt.name.clone(),
            target_type: TomlBuildTargetType::from_runtime(&rt.target_type),
            tex_format,
            preamble_file,
            index_file,
            postamble_file,
            shell_escape,
            shell_escape_cwd,
        }
    }

    pub fn to_runtime(&self) -> OutputProfile {
        let shell_escape_default = self.shell_escape_cwd.is_some();

        OutputProfile {
            name: self.name.clone(),
            target_type: self.target_type.to_runtime(),
            tex_format: self
                .tex_format
                .as_ref()
                .map(|s| s.as_ref())
                .unwrap_or("latex")
                .to_owned(),
            preamble_file: self
                .preamble_file
                .clone()
                .unwrap_or_else(|| DEFAULT_PREAMBLE_FILE.to_owned()),
            index_file: self
                .index_file
                .clone()
                .unwrap_or_else(|| DEFAULT_INDEX_FILE.to_owned()),
            postamble_file: self
                .postamble_file
                .clone()
                .unwrap_or_else(|| DEFAULT_POSTAMBLE_FILE.to_owned()),
            shell_escape: self.shell_escape.unwrap_or(shell_escape_default),
            shell_escape_cwd: self.shell_escape_cwd.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TomlBuildTargetType {
    Html,
    Pdf,
}

impl TomlBuildTargetType {
    pub fn from_runtime(rt: &BuildTargetType) -> Self {
        match rt {
            BuildTargetType::Html => TomlBuildTargetType::Html,
            BuildTargetType::Pdf => TomlBuildTargetType::Pdf,
        }
    }

    pub fn to_runtime(self) -> BuildTargetType {
        match self {
            TomlBuildTargetType::Html => BuildTargetType::Html,
            TomlBuildTargetType::Pdf => BuildTargetType::Pdf,
        }
    }
}

impl Serialize for TomlBuildTargetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            TomlBuildTargetType::Html => "html",
            TomlBuildTargetType::Pdf => "pdf",
        })
    }
}
impl<'de> Deserialize<'de> for TomlBuildTargetType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "html" => TomlBuildTargetType::Html,
            "pdf" => TomlBuildTargetType::Pdf,
            other => {
                return Err(<D as Deserializer>::Error::unknown_variant(
                    other,
                    &["html", "pdf"],
                ))
            }
        })
    }
}
