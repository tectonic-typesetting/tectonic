mod dir;
mod tar;

use anyhow::Result;
use std::{io::Read, path::PathBuf};

trait BundleInput {
    #[allow(clippy::type_complexity)]
    fn iter_files(&mut self) -> impl Iterator<Item = Result<(String, Box<dyn Read + '_>)>>;
}

pub enum Input {
    Directory(dir::DirBundleInput),
    Tarball(tar::TarBundleInput),
}

impl<'a> Input {
    pub fn new_dir(path: PathBuf) -> Self {
        Self::Directory(dir::DirBundleInput::new(path))
    }

    pub fn new_tarball(path: PathBuf, root: Option<PathBuf>) -> Result<Self> {
        Ok(Self::Tarball(tar::TarBundleInput::new(path, root)?))
    }

    #[allow(clippy::type_complexity)]
    pub fn iter_files(
        &'a mut self,
    ) -> Box<dyn Iterator<Item = Result<(String, Box<dyn Read + 'a>)>> + 'a> {
        match self {
            Self::Directory(x) => Box::new(x.iter_files()),
            Self::Tarball(x) => Box::new(x.iter_files()),
        }
    }

    pub fn hash(&self) -> Option<&str> {
        match self {
            Self::Directory(_) => None,
            Self::Tarball(x) => Some(x.hash()),
        }
    }
}
