// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Learned working-set prefetching for cached bundles.

use super::{file_create_write, BundleCache};
use crate::FileIndex;
use std::{
    collections::HashSet,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::Path,
};
use tectonic_io_base::OpenResult;
use tectonic_status_base::{tt_note, StatusBackend};

/// Upper bound on the number of file names recorded in a prefetch manifest.
const MAX_MANIFEST_ENTRIES: usize = 512;

/// Maximum number of file bodies retained in memory by one batch.
const MAX_PREFETCH_BATCH_FILES: usize = 64;

fn manifest_name_is_valid(name: &str) -> bool {
    !name.is_empty() && !name.as_bytes().iter().any(|b| matches!(b, b'\r' | b'\n'))
}

fn parse_manifest(reader: impl BufRead) -> HashSet<String> {
    let mut out = HashSet::new();

    for line in reader.lines().map_while(Result::ok) {
        if !manifest_name_is_valid(&line) {
            continue;
        }

        if out.insert(line.to_owned()) && out.len() == MAX_MANIFEST_ENTRIES {
            break;
        }
    }

    out
}

/// Load a prefetch manifest. Missing or unreadable manifests simply disable
/// the optimization for this run.
pub(super) fn load_manifest(path: &Path) -> HashSet<String> {
    File::open(path)
        .map(|f| parse_manifest(BufReader::new(f)))
        .unwrap_or_default()
}

impl<'this, T: FileIndex<'this>> BundleCache<'this, T> {
    /// Warm a learned working set when its metadata remains but file blobs do not.
    pub(super) fn prefetch(&mut self, status: &mut dyn StatusBackend) {
        if !self.prefetch_supported
            || self.prefetched
            || self.only_cached
            || self.touched.is_empty()
        {
            return;
        }
        self.prefetched = true;

        if let Err(e) = self.ensure_index() {
            tt_note!(status, "skipping prefetch, couldn't load bundle index: {e}");
            return;
        }

        let mut names: Vec<String> = self.touched.iter().cloned().collect();
        names.sort();
        let mut infos = Vec::new();
        for name in &names {
            if let Some(info) = self.bundle.search(name) {
                if !self.get_file_path(&info).exists() {
                    infos.push(info);
                }
            }
        }

        for batch in infos.chunks(MAX_PREFETCH_BATCH_FILES) {
            let results = self.bundle.batch_open(batch, status);
            let mut complete = results.len() == batch.len()
                && results
                    .iter()
                    .all(|result| matches!(result, OpenResult::Ok(_)));

            for (info, result) in batch.iter().zip(results) {
                let OpenResult::Ok(bytes) = result else {
                    continue;
                };

                let target = self.get_file_path(info);
                if fs::create_dir_all(target.parent().unwrap()).is_err() {
                    complete = false;
                    continue;
                }

                let tmp_path = self.get_file_path_tmp(info);
                if file_create_write(&tmp_path, |f| f.write_all(&bytes)).is_err() {
                    let _ = fs::remove_file(&tmp_path);
                    complete = false;
                    continue;
                }

                if fs::rename(&tmp_path, &target).is_err() {
                    let _ = fs::remove_file(&tmp_path);
                    if !target.exists() {
                        complete = false;
                    }
                }
            }

            if !complete {
                break;
            }
        }
    }
}

impl<T> BundleCache<'_, T> {
    /// Remember a successfully resolved lookup for a future prefetch.
    pub(super) fn record_resolved_name(&mut self, name: &str) {
        if !self.prefetch_supported
            || !manifest_name_is_valid(name)
            || self.touched.len() >= MAX_MANIFEST_ENTRIES
            || !self.touched.insert(name.to_owned())
        {
            return;
        }

        let mut line = Vec::with_capacity(name.len() + 1);
        line.extend_from_slice(name.as_bytes());
        line.push(b'\n');

        if let Ok(mut file) = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.manifest_path)
        {
            let _ = file.write_all(&line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn manifest_parser_skips_blanks_and_deduplicates() {
        let input = Cursor::new(b"\nalpha\r\nalpha\nbeta\nbad\rname\n");
        let parsed = parse_manifest(input);

        assert_eq!(parsed.len(), 2);
        assert!(parsed.contains("alpha"));
        assert!(parsed.contains("beta"));
    }

    #[test]
    fn manifest_parser_caps_unique_entries() {
        let mut input = String::from("duplicate\nduplicate\n\n");
        for i in 0..MAX_MANIFEST_ENTRIES {
            input.push_str(&format!("entry-{i}\n"));
        }
        input.push_str("past-the-cap\n");

        let parsed = parse_manifest(Cursor::new(input));

        assert_eq!(parsed.len(), MAX_MANIFEST_ENTRIES);
        assert!(parsed.contains("duplicate"));
        assert!(parsed.contains(&format!("entry-{}", MAX_MANIFEST_ENTRIES - 2)));
        assert!(!parsed.contains(&format!("entry-{}", MAX_MANIFEST_ENTRIES - 1)));
        assert!(!parsed.contains("past-the-cap"));
    }
}
