// src/io/itarbundle.rs -- I/O on files in an indexed tar file "bundle"
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

use flate2::read::GzDecoder;
use hyper::{self, Client};
use hyper::client::Response;
use hyper::header::{Headers, Range};
use hyper::status::StatusCode;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::io::{BufRead, BufReader, Cursor, Read};

use errors::Result;
use io::{InputHandle, IoProvider, OpenResult};
use status::StatusBackend;


// A simple way to read chunks out of a big seekable byte stream. You could
// implement this for io::File pretty trivially but that's not currently
// needed.

pub trait RangeRead {
    type InnerRead: Read;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Self::InnerRead>;
}


pub struct HttpRangeReader {
    url: String,
    client: Client,
}


impl HttpRangeReader {
    pub fn new(url: &str) -> HttpRangeReader {
        let client = Client::new();

        HttpRangeReader {
            url: url.to_owned(),
            client: client,
        }
    }
}


impl RangeRead for HttpRangeReader {
    type InnerRead = Response;

    fn read_range(&mut self, offset: u64, length: usize) -> Result<Response> {
        let end_inclusive = offset + length as u64 - 1;

        let mut headers = Headers::new();
        headers.set(Range::bytes(offset, end_inclusive));

        let req = self.client.get(&self.url).headers(headers);
        let res = req.send()?;

        if res.status != StatusCode::PartialContent {
            return Err(hyper::Error::Status.into());
        }

        Ok(res)
    }
}


// The IoProvider. We jump through some hoops so that web-based bundles can
// be created without immediately connecting to the network.

pub trait ITarIoFactory {
    type IndexReader: Read;
    type DataReader: RangeRead;

    fn get_index(&self) -> Result<Self::IndexReader>;
    fn get_data(&self) -> Result<Self::DataReader>;
}

struct FileInfo {
    offset: u64,
    length: u64
}

pub struct ITarBundle<F: ITarIoFactory> {
    factory: F,
    data: Option<F::DataReader>,
    index: HashMap<OsString,FileInfo>,
}


impl<F: ITarIoFactory> ITarBundle<F> {
    fn construct (factory: F) -> ITarBundle<F> {
        ITarBundle {
            factory: factory,
            data: None,
            index: HashMap::new(),
        }
    }

    fn ensure_loaded(&mut self) -> Result<()> {
        if self.data.is_some() {
            return Ok(());
        }

        // We need to initialize. First, the index ...

        let index = self.factory.get_index()?;
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
            self.index.insert(name, FileInfo { offset: offset, length: length });
        }

        // ... then, the data reader.

        self.data = Some(self.factory.get_data()?);
        Ok(())
    }
}


impl<F: ITarIoFactory> IoProvider for ITarBundle<F> {
    fn input_open_name(&mut self, name: &OsStr, _status: &mut StatusBackend) -> OpenResult<InputHandle> {
        if let Err(e) = self.ensure_loaded() {
            return OpenResult::Err(e.into());
        }

        // In principle it'd be cool to return a handle right to the HTTP
        // response, but those can't be seekable, and doing so introduces
        // lifetime-related issues. So for now we just slurp the whole thing
        // into RAM.

        let info = match self.index.get (name) {
            Some(i) => i,
            None => return OpenResult::NotAvailable,
        };

        let mut stream = match self.data.as_mut().unwrap().read_range(info.offset, info.length as usize) {
            Ok(r) => r,
            Err(e) => return OpenResult::Err(e.into())
        };

        let mut buf = Vec::with_capacity(info.length as usize);
        if let Err(e) = stream.read_to_end(&mut buf) {
            return OpenResult::Err(e.into());
        }

        OpenResult::Ok(Box::new(Cursor::new(buf)))
    }
}


pub struct HttpITarIoFactory {
    url: String,
}

impl ITarIoFactory for HttpITarIoFactory {
    type IndexReader = GzDecoder<Response>;
    type DataReader = HttpRangeReader;

    fn get_index(&self) -> Result<GzDecoder<Response>> {
        let mut index_url = self.url.clone();
        index_url.push_str(".index.gz");

        let client = Client::new();
        let req = client.get(&index_url);
        let res = req.send()?;
        if res.status != StatusCode::Ok {
            return Err(hyper::Error::Status.into());
        }

        Ok(GzDecoder::new(res)?) // <- needed to convert Error types
    }

    fn get_data(&self) -> Result<HttpRangeReader> {
        Ok(HttpRangeReader::new(&self.url))
    }
}

impl ITarBundle<HttpITarIoFactory> {
    pub fn new (url: &str) -> ITarBundle<HttpITarIoFactory> {
        Self::construct(HttpITarIoFactory { url: url.to_owned() })
    }
}
