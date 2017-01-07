// src/itarbundle.rs -- I/O on files in an indexed tar file "bundle"
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use hyper::{self, Client};
use hyper::status::StatusCode;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom};

use errors::Result;
use hyper_seekable::SeekableHTTPFile;
use io::{InputHandle, IOProvider, OpenResult};


struct FileInfo {
    offset: u64,
    length: u64
}

pub struct ITarBundle<R: Read + Seek> {
    data: R,
    index: HashMap<OsString,FileInfo>,
}


impl<R: Read + Seek> ITarBundle<R> {
    pub fn new<RI: Read> (index: RI, data: R) -> Result<ITarBundle<R>> {
        let mut bundle = ITarBundle {
            data: data,
            index: HashMap::new(),
        };

        let br = BufReader::new(index);

        for res in br.lines() {
            let line = res?;
            let bits = line.split_whitespace().collect::<Vec<_>>();

            if bits.len() < 3 {
                continue; // TODO: preserve the warning info or something!
            }

            let name = OsString::from(bits[0]);
            let offset = bits[1].parse::<u64>()?;
            let length = bits[2].parse::<u64>()?;
            bundle.index.insert(name, FileInfo { offset: offset, length: length });
        }

        Ok(bundle)
    }
}

impl ITarBundle<SeekableHTTPFile> {
    pub fn open (url: &str) -> Result<ITarBundle<SeekableHTTPFile>> {
        // Set up to stream the index.

        let mut index_url = String::from(url);
        index_url.push_str(".index.gz");

        let client = Client::new();
        let req = client.get(&index_url);
        let res = req.send()?;
        if res.status != StatusCode::Ok {
            return Err(hyper::Error::Status.into());
        }

        let gzindex = GzDecoder::new(res)?;

        // Ready to pass off.
        Self::new(gzindex, SeekableHTTPFile::new(url)?)
    }
}


impl<R: Read + Seek> IOProvider for ITarBundle<R> {
    fn input_open_name(&mut self, name: &OsStr) -> OpenResult<InputHandle> {
        // We need to be able to look at other items in the bundle while
        // reading this one. In principle we could do this with multiple
        // simultaneous HTTP Range requests in the HTTP case, but it's nice to
        // have us just rely on the data backend being Read+Seek. In which
        // case we need to read in the whole file immediately. RAM is cheap.

        let info = match self.index.get (name) {
            Some(i) => i,
            None => return OpenResult::NotAvailable,
        };

        if let Err(e) = self.data.seek(SeekFrom::Start(info.offset)) {
            return OpenResult::Err(e.into());
        }

        let mut buf = Vec::with_capacity(info.length as usize);
        if let Err(e) = (&mut self.data).take(info.length).read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(Box::new(Cursor::new(buf)))
    }
}
