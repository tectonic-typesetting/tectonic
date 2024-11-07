use anyhow::{bail, Result};
use flate2::{write::GzEncoder, Compression};
use std::{
    fmt::Display,
    fs::{self, File},
    io::{stdout, BufRead, BufReader, Read, Seek, Write},
    num::ParseIntError,
    path::PathBuf,
};
use tracing::info;

pub trait WriteSeek: std::io::Write + Seek {}
impl<T: Write + Seek> WriteSeek for T {}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

// Size of ttbv1 header
const HEADER_SIZE: u64 = 66u64;

#[derive(Debug)]
struct FileListEntry {
    path: PathBuf,
    hash: String,
    start: u64,

    // We need the compressed length to build
    // a range request for this bundle. We also
    // keep the real length around for performance
    // (we'll only need to allocate vectors once)
    real_len: u32,
    gzip_len: u32,
}

impl Display for FileListEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format!(
            "{} {} {} {} {}",
            self.start,
            self.gzip_len,
            self.real_len,
            self.hash,
            self.path.to_str().unwrap()
        )
        .fmt(f)
    }
}

pub struct BundleV1 {
    filelist: Vec<FileListEntry>,
    target: Box<dyn WriteSeek>,
    content_dir: PathBuf,

    index_start: u64,
    index_real_len: u32,
    index_gzip_len: u32,
}

impl BundleV1 {
    pub fn make(target: Box<dyn WriteSeek>, build_dir: PathBuf) -> Result<()> {
        let mut bundle = BundleV1::new(target, build_dir)?;

        bundle.add_files()?;
        bundle.write_index()?;
        bundle.write_header()?;

        Ok(())
    }

    fn new(target: Box<dyn WriteSeek>, build_dir: PathBuf) -> Result<BundleV1> {
        Ok(BundleV1 {
            filelist: Vec::new(),
            target,
            content_dir: build_dir.join("content"),
            index_start: 0,
            index_gzip_len: 0,
            index_real_len: 0,
        })
    }

    fn add_files(&mut self) -> Result<u64> {
        let mut byte_count = HEADER_SIZE; // Start after header
        let mut real_len_sum = 0; // Compute average compression ratio

        self.target.seek(std::io::SeekFrom::Start(byte_count))?;

        let filelist_file = File::open(self.content_dir.join("FILELIST"))?;
        let reader = BufReader::new(filelist_file);

        info!("Building ttbv1 bundle...");

        for line in reader.lines() {
            stdout().flush()?;

            let line = line?;
            let mut bits = line.split_whitespace();

            if let Some(hash) = bits.next() {
                let path = bits.collect::<Vec<&str>>().join(" ");

                let mut file = fs::File::open(self.content_dir.join(&path))?;

                // Compress and write bytes
                let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
                let real_len = std::io::copy(&mut file, &mut encoder)?;
                let gzip_len = self.target.write(&encoder.finish()?)?;
                assert!(real_len < u32::MAX as u64);
                assert!(gzip_len < u32::MAX as usize);

                // Add to index
                self.filelist.push(FileListEntry {
                    start: byte_count,
                    gzip_len: gzip_len as u32,
                    real_len: real_len as u32,
                    path: PathBuf::from(path),
                    hash: hash.to_owned(),
                });
                byte_count += gzip_len as u64;
                real_len_sum += real_len;
            } else {
                bail!("malformed filelist line");
            }
        }

        info!(
            "Average compression ratio: {:.2}",
            real_len_sum as f64 / byte_count as f64
        );

        Ok(byte_count)
    }

    fn write_index(&mut self) -> Result<()> {
        // Generate a ttbv1 index and write it to the bundle.
        //
        // This index is a replacement for FILELIST and SEARCH, containing everything in those files
        // (in addition to some ttbv1-specific information)
        //
        // The original FILELIST and SEARCH files are still included in the bundle.

        // Get current position
        self.index_start = self.target.stream_position()?;

        info!("Writing index");

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        let mut real_len = 0usize;

        real_len += encoder.write("[DEFAULTSEARCH]\n".as_bytes())?;
        real_len += encoder.write("MAIN\n".as_bytes())?;

        real_len += encoder.write("[SEARCH:MAIN]\n".as_bytes())?;
        for l in fs::read_to_string(self.content_dir.join("SEARCH"))?.lines() {
            real_len += encoder.write(l.as_bytes())?;
            real_len += encoder.write(b"\n")?;
        }

        real_len += encoder.write("[FILELIST]\n".as_bytes())?;
        for i in &self.filelist {
            let s = format!("{i}\n");
            real_len += encoder.write(s.as_bytes())?;
        }
        let gzip_len = self.target.write(&encoder.finish()?)?;
        assert!(gzip_len < u32::MAX as usize);
        assert!(real_len < u32::MAX as usize);
        self.index_gzip_len = gzip_len as u32;
        self.index_real_len = real_len as u32;

        info!(
            "index is at {} and has length {}",
            self.index_start, self.index_gzip_len
        );

        Ok(())
    }

    fn write_header(&mut self) -> Result<u64> {
        self.target.seek(std::io::SeekFrom::Start(0))?;

        info!("Writing header");

        // Parse bundle hash
        let mut hash_file = File::open(self.content_dir.join("SHA256SUM")).unwrap();
        let mut hash_text = String::new();
        hash_file.read_to_string(&mut hash_text)?;
        let digest = decode_hex(hash_text.trim())?;

        let mut byte_count = 0u64;

        // 14 bytes: signature
        // Always "tectonicbundle", in any bundle version.
        //
        // This "magic sequence" lets us more easily distinguish between
        // random binary files and proper tectonic bundles.
        byte_count += self.target.write(b"tectonicbundle")? as u64;

        // 4 bytes: bundle version
        byte_count += self.target.write(&1u32.to_le_bytes())? as u64;

        // 8 + 4 + 4 = 12 bytes: location and real length of index
        byte_count += self.target.write(&self.index_start.to_le_bytes())? as u64;
        byte_count += self.target.write(&self.index_gzip_len.to_le_bytes())? as u64;
        byte_count += self.target.write(&self.index_real_len.to_le_bytes())? as u64;

        // 32 bytes: bundle hash
        // We include this in the header so we don't need to load the index to get the hash.
        byte_count += self.target.write(&digest)? as u64;

        // Make sure we wrote the expected number of bytes
        assert!(byte_count == HEADER_SIZE);

        Ok(byte_count)
    }
}
