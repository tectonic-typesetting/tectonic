// src/hyper_seekable.rs -- seekable remote file over HTTP
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

const CHUNK_SIZE: usize = 4096;

type Chunk = [u8; CHUNK_SIZE];

pub struct SeekableHTTPStream {
    url: String,
    client: Client,
    cache: HashMap<u64, Chunk>,
    pos: u64,
    len: u64,
}


impl SeekableHTTPStream {
    pub fn new(url: &str) -> Result<SeekableHTTPStream> {
        let client = Client::new();

        let len = {
            let req = client.head(url);
            let res = req.send()?;
            match res.headers.get::<ContentLength>() {
                Some(cl) => cl.0,
                None => return Err(ErrorKind::NotSizeable.into())
            }
        };

        Ok(SeekableHTTPStream {
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
}


impl Read for SeekableHTTPStream {
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


impl Seek for SeekableHTTPStream {
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
                if ofs > 0 {
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
    use std::io::{Read, Write};
    use std::net::SocketAddr;

    fn with_server_thread<H, F, T>(h: H, f: F) -> T where H: Handler + 'static, F: FnOnce (SocketAddr) -> T {
        let mut server = Server::http("127.0.0.1:0").unwrap().handle(h).unwrap();
        let retval = f(server.socket);
        server.close().expect("error stopping embedded test server");
        retval
    }

    fn with_connected_shs<H, F, T>(h: H, f: F) -> T where H: Handler + 'static,
                                                          F: FnOnce (&mut SeekableHTTPStream) -> T {
        with_server_thread(h, |sock| {
            let url = format!("http://{}:{}/", sock.ip(), sock.port());
            let mut shs = SeekableHTTPStream::new(&url).unwrap();
            f(&mut shs)
        })
    }

    fn with_fixed_buf_server<F, T>(buf: &'static [u8], f: F) -> T where F: FnOnce (&mut SeekableHTTPStream) -> T {
        let sz = buf.len() as u64;

        with_connected_shs(move |req: Request, mut res: Response| {
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
        with_fixed_buf_server(HELLO_WORLD, |shs| {
            assert_eq!(shs.len(), 12);

            let mut full = Vec::new();
            shs.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], HELLO_WORLD);
        });
    }


    static TWO_CHUNKS: &'static [u8] = &[0; CHUNK_SIZE + 1];

    #[test]
    fn two_chunks() {
        with_fixed_buf_server(TWO_CHUNKS, |shs| {
            assert_eq!(shs.len(), CHUNK_SIZE as u64 + 1);

            let mut full = Vec::new();
            shs.read_to_end(&mut full).unwrap();
            assert_eq!(&full[..], TWO_CHUNKS);
        });
    }
}
