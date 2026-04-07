use super::BundleInput;
use anyhow::Result;
use std::{
    fs::{self},
    io::Read,
    path::PathBuf,
};
use walkdir::WalkDir;

pub struct DirBundleInput {
    dir: PathBuf,
}

impl DirBundleInput {
    pub fn new(dir: PathBuf) -> Self {
        Self {
            dir: dir.canonicalize().unwrap(),
        }
    }
}

impl BundleInput for DirBundleInput {
    fn iter_files(&mut self) -> impl Iterator<Item = Result<(String, Box<dyn Read + '_>)>> {
        WalkDir::new(&self.dir)
            .into_iter()
            .filter_map(|x| match x {
                Err(_) => Some(x),
                Ok(x) => {
                    if !x.file_type().is_file() {
                        None
                    } else {
                        Some(Ok(x))
                    }
                }
            })
            .map(move |x| match x {
                Ok(x) => {
                    let path = x
                        .into_path()
                        .canonicalize()
                        .unwrap()
                        .strip_prefix(&self.dir)
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string();

                    Ok((
                        path.clone(),
                        Box::new(fs::File::open(self.dir.join(path))?) as Box<dyn Read>,
                    ))
                }
                Err(e) => Err(anyhow::Error::from(e)),
            })
    }
}
