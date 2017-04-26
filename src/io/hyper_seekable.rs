// src/io/hyper_seekable.rs -- seekable remote file over HTTP
// Copyright 2017 the Tectonic Project
// Licensed under the MIT License.

// TODO: this should probably be its own crate, but I'm writing this on an
// airplane and I can't get that set up without a web connection.

use hyper::Client;
use hyper::header::{Headers, ContentLength, Range};
use hyper::status::StatusCode;
use std::cmp;
use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom};
use std::str;

use errors::{ErrorKind, Result};

// Of course, we are not actually caching because nothing ever gets expired
// from `cache`. TBD.

const CHUNK_SIZE: usize = 8192;

type Chunk = [u8; CHUNK_SIZE];

pub struct SeekableHttpFile {
    url: String,
    client: Client,
    cache: HashMap<u64, Chunk>,
    pos: u64,
    len: u64,
}


impl SeekableHttpFile {
    pub fn new(url: &str) -> Result<SeekableHttpFile> {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);

        let len = {
            let req = client.head(url);
            let res = req.send()?;
            match res.headers.get::<ContentLength>() {
                Some(cl) => cl.0,
                None => return Err(ErrorKind::NotSizeable.into())
            }
        };

        Ok(SeekableHttpFile {
            url: url.to_owned(),
            client: client,
            cache: HashMap::new(),
            pos: 0,
            len: len,
        })
    }

    pub fn len(&self) -> u64 {
        self.len
    }

    pub fn drop_cache(&mut self) {
        self.cache.clear()
    }
}


impl Read for SeekableHttpFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let idx = self.pos / CHUNK_SIZE as u64;
        let ofs = (self.pos % CHUNK_SIZE as u64) as usize;
        let len = cmp::min(cmp::min(CHUNK_SIZE - ofs, buf.len()),
                           (self.len - self.pos) as usize);

        if len == 0 {
            return Ok(0)
        }

        // NB I tried to pull out the common bits in the two code paths here
        // but ran into borrow-checker problems.

        if let Some(cbuf) = self.cache.get(&idx) {
            buf[..len].copy_from_slice(&cbuf[ofs..ofs+len]);
            self.pos += len as u64;
            return Ok(len as usize);
        }

        // We need to fetch this chunk. All chunks are of size CHUNK_SIZE
        // except the annoying last one.
        let start = idx * CHUNK_SIZE as u64;
        let end_inclusive = cmp::min(start + CHUNK_SIZE as u64 - 1,
                                     self.len - 1);
        let chunk_size = (end_inclusive + 1 - start) as usize;
        println!("FETCHING {} -> {} (inclusive)", start, end_inclusive);

        let mut headers = Headers::new();
        headers.set(Range::bytes(start, end_inclusive));

        let req = self.client.get(&self.url).headers(headers);

        let mut res = match req.send() {
            Ok(r) => r,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other,
                                                format!("{}", e))),
        };
        if res.status != StatusCode::PartialContent {
            return Err(io::Error::new(io::ErrorKind::Other,
                                      format!("unexpected status {}", res.status)));
        }

        let mut chunk: Chunk = [0u8; CHUNK_SIZE as usize];
        res.read_exact(&mut chunk[..chunk_size])?;

        buf[..len].copy_from_slice(&chunk[ofs..ofs+len]);
        self.cache.insert(idx, chunk);
        self.pos += len as u64;
        Ok(len as usize)
    }
}


impl Seek for SeekableHttpFile {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let tot_len = self.len as i64;
        let cur = self.pos as i64;

        self.pos = match pos {
            SeekFrom::Start(p) => {
                if p > tot_len as u64 {
                    return Err(io::Error::new(io::ErrorKind::Other, "out-of-bounds seek"));
                }
                p
            },
            SeekFrom::End(ofs) => {
                if ofs > 0 || -ofs > tot_len {
                    return Err(io::Error::new(io::ErrorKind::Other, "out-of-bounds seek"));
                }
                (tot_len + ofs) as u64
            },
            SeekFrom::Current(ofs) => {
                if -ofs > cur || ofs + cur > tot_len {
                    return Err(io::Error::new(io::ErrorKind::Other, "out-of-bounds seek"));
                }
                (cur + ofs) as u64
            }
        };

        println!("OK SEEK: {}", self.pos);
        Ok(self.pos)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::CHUNK_SIZE;
    use hyper::header::{ByteRangeSpec, ContentLength, Range};
    use hyper::method::Method;
    use hyper::server::{Handler, Response, Request, Server};
    use hyper::status::StatusCode;
    use std::io::{Read, Seek, SeekFrom, Write};
    use std::net::SocketAddr;

    fn with_server_thread<H, F, T>(h: H, f: F) -> T where H: Handler + 'static, F: FnOnce (SocketAddr) -> T {
        let mut server = Server::http("127.0.0.1:0").unwrap().handle(h).unwrap();
        let retval = f(server.socket);
        server.close().expect("error stopping embedded test server");
        retval
    }

    fn with_connected_shf<H, F, T>(h: H, f: F) -> T where H: Handler + 'static,
                                                          F: FnOnce (&mut SeekableHttpFile) -> T {
        with_server_thread(h, |sock| {
            let url = format!("http://{}:{}/", sock.ip(), sock.port());
            let mut shf = SeekableHttpFile::new(&url).unwrap();
            f(&mut shf)
        })
    }

    fn with_fixed_buf_server<F, T>(buf: &'static [u8], f: F) -> T where F: FnOnce (&mut SeekableHttpFile) -> T {
        let sz = buf.len() as u64;

        with_connected_shf(move |req: Request, mut res: Response| {
            match req.method {
                Method::Head => {
                    res.headers_mut().set(ContentLength(buf.len() as u64));
                    res.start().unwrap();
                    // No body content.
                },
                Method::Get => {
                    if let Some(range) = req.headers.get::<Range>() {
                        match range {
                            &Range::Unregistered(..) => unimplemented!(),
                            &Range::Bytes(ref specvec) => {
                                if specvec.len() != 1 {
                                    unimplemented!();
                                }

                                match specvec[0] {
                                    // `start` is unsigned so it can never be negative.
                                    ByteRangeSpec::FromTo(start, end) => {
                                        // TODO?: incomplete writes, etc?
                                        assert!(end >= start);
                                        assert!(end < sz); // end index is inclusive
                                        *res.status_mut() = StatusCode::PartialContent;
                                        res.headers_mut().set(ContentLength(end + 1 - start));
                                        let mut res = res.start().unwrap();
                                        res.write_all(&buf[start as usize..(end+1) as usize]).unwrap();
                                    },
                                    ByteRangeSpec::AllFrom(_start) => unimplemented!(),
                                    ByteRangeSpec::Last(_end) => unimplemented!(),
                                }
                            },
                        }
                    } else {
                        res.send(buf).unwrap();
                    }
                },
                _ => *res.status_mut() = StatusCode::MethodNotAllowed
            }
        }, f)
    }


    static HELLO_WORLD: &'static [u8] = b"Hello world!";

    #[test]
    fn hello_world() {
        with_fixed_buf_server(HELLO_WORLD, |shf| {
            assert_eq!(shf.len(), 12);

            let mut full = Vec::new();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], HELLO_WORLD);

            full.clear();
            shf.seek(SeekFrom::Start(0)).unwrap();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], HELLO_WORLD);

            full.clear();
            shf.seek(SeekFrom::Start(10)).unwrap();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], b"d!");

            full.clear();
            shf.seek(SeekFrom::End(-4)).unwrap();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], b"rld!");

            let mut four = [0u8; 4];
            shf.seek(SeekFrom::Start(2)).unwrap();
            shf.read_exact(&mut four).unwrap();
            assert_eq!(&four, b"llo ");

            shf.seek(SeekFrom::Current(-2)).unwrap();
            shf.read_exact(&mut four).unwrap();
            assert_eq!(&four, b"o wo");

            full.clear();
            shf.seek(SeekFrom::Current(2)).unwrap();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], b"d!");
        });
    }

    #[test]
    fn hello_world_seeks() {
        // I wanted these to be #[should_panic] tests, but if it panics
        // the test fails ...
        with_fixed_buf_server(HELLO_WORLD, |shf| {
            assert_eq!(shf.len(), 12);
            shf.seek(SeekFrom::Start(0)).unwrap();
            shf.seek(SeekFrom::Start(12)).unwrap();
            shf.seek(SeekFrom::Start(13)).unwrap_err();

            shf.seek(SeekFrom::End(0)).unwrap();
            shf.seek(SeekFrom::End(1)).unwrap_err();
            shf.seek(SeekFrom::End(-12)).unwrap();
            shf.seek(SeekFrom::End(-13)).unwrap_err();

            shf.seek(SeekFrom::Start(6)).unwrap();
            shf.seek(SeekFrom::Current(-6)).unwrap();
            shf.seek(SeekFrom::Start(6)).unwrap();
            shf.seek(SeekFrom::Current(-7)).unwrap_err();
            shf.seek(SeekFrom::Start(6)).unwrap();
            shf.seek(SeekFrom::Current(6)).unwrap();
            shf.seek(SeekFrom::Start(6)).unwrap();
            shf.seek(SeekFrom::Current(7)).unwrap_err();
        });
    }


    static TWO_CHUNKS: &'static [u8] = &[0; CHUNK_SIZE + 1];

    #[test]
    fn two_chunks() {
        with_fixed_buf_server(TWO_CHUNKS, |shf| {
            assert_eq!(shf.len(), CHUNK_SIZE as u64 + 1);

            let mut full = Vec::new();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], TWO_CHUNKS);

            shf.seek(SeekFrom::Start(0)).unwrap();
            full.clear();
            shf.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], TWO_CHUNKS);
        });
    }
}
