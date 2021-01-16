use std::{fs::File, io::BufReader, path::PathBuf};

use super::{Bundle, InputHandle, InputOrigin, IoProvider, OpenResult};
use crate::status::StatusBackend;

pub struct DirBundle {
    dir: PathBuf,
}

impl DirBundle {
    pub fn new(dir: PathBuf) -> DirBundle {
        DirBundle { dir }
    }
}

impl IoProvider for DirBundle {
    fn input_open_name(
        &mut self,
        name: &str,
        _status: &mut dyn StatusBackend,
    ) -> OpenResult<InputHandle> {
        let mut path = self.dir.clone();
        path.push(name);

        if path.is_file() {
            match File::open(path) {
                Err(e) => OpenResult::Err(e.into()),
                Ok(f) => OpenResult::Ok(InputHandle::new(
                    name,
                    BufReader::new(f),
                    InputOrigin::Filesystem,
                )),
            }
        } else {
            OpenResult::NotAvailable
        }
    }
}

impl Bundle for DirBundle {}
