// Copyright 2017-2021 the Tectonic Project
// Licensed under the MIT License.

//! Concurrent byte-range reads for indexed-tar bundles.

use super::{read_file_with_retries, ItarBundle, ItarFileInfo};
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
    thread,
};
use tectonic_errors::prelude::*;
use tectonic_geturl::{DefaultBackend, GetUrlBackend};
use tectonic_io_base::OpenResult;
use tectonic_status_base::{tt_note, NoopStatusBackend, StatusBackend};

const DEFAULT_PREFETCH_CONCURRENCY: usize = 16;
const MAX_PREFETCH_CONCURRENCY: usize = 64;
const MAX_PREFETCH_BYTES: usize = 64 * 1024 * 1024;

fn prefetch_concurrency(value: Option<&str>, file_count: usize) -> usize {
    if file_count == 0 {
        return 0;
    }

    value
        .and_then(|s| s.parse::<usize>().ok())
        .filter(|n| *n > 0)
        .unwrap_or(DEFAULT_PREFETCH_CONCURRENCY)
        .min(MAX_PREFETCH_CONCURRENCY)
        .min(file_count)
}

fn prefetch_indices(infos: &[ItarFileInfo]) -> Vec<usize> {
    let mut bytes = 0usize;
    let mut selected = Vec::new();

    for (index, info) in infos.iter().enumerate() {
        if let Some(next_bytes) = bytes.checked_add(info.length) {
            if next_bytes <= MAX_PREFETCH_BYTES {
                bytes = next_bytes;
                selected.push(index);
            }
        }
    }

    selected
}

pub(super) fn open(
    bundle: &mut ItarBundle,
    infos: &[ItarFileInfo],
    status: &mut dyn StatusBackend,
) -> Vec<OpenResult<Vec<u8>>> {
    if infos.is_empty() {
        return Vec::new();
    }

    let selected = prefetch_indices(infos);
    if selected.is_empty() {
        return infos.iter().map(|_| OpenResult::NotAvailable).collect();
    }

    if let Err(e) = bundle.ensure_index() {
        return infos
            .iter()
            .map(|_| OpenResult::Err(anyhow!("failed to load bundle index: {e}")))
            .collect();
    }

    let configured = std::env::var("TECTONIC_PREFETCH_CONCURRENCY").ok();
    let concurrency = prefetch_concurrency(configured.as_deref(), selected.len());
    tt_note!(
        status,
        "prefetching {} of {} files ({}-way concurrent)",
        selected.len(),
        infos.len(),
        concurrency
    );

    let url = &bundle.url;
    let next = AtomicUsize::new(0);
    let abort = AtomicBool::new(false);
    let result_slots: Vec<Mutex<OpenResult<Vec<u8>>>> = (0..infos.len())
        .map(|_| Mutex::new(OpenResult::NotAvailable))
        .collect();

    thread::scope(|scope| {
        let mut workers = Vec::with_capacity(concurrency);
        for _ in 0..concurrency {
            let worker = thread::Builder::new().spawn_scoped(scope, || {
                let mut reader = DefaultBackend::default().open_range_reader(url);
                let mut status = NoopStatusBackend {};

                loop {
                    if abort.load(Ordering::Relaxed) {
                        break;
                    }

                    let selected_index = next.fetch_add(1, Ordering::Relaxed);
                    if selected_index >= selected.len() {
                        break;
                    }
                    let i = selected[selected_index];

                    let result = read_file_with_retries(&mut reader, &infos[i], &mut status);
                    if !matches!(result, OpenResult::Ok(_)) {
                        abort.store(true, Ordering::Relaxed);
                    }
                    if let Ok(mut slot) = result_slots[i].lock() {
                        *slot = result;
                    }
                }
            });

            match worker {
                Ok(worker) => workers.push(worker),
                Err(_) => break,
            }
        }

        // A failed worker leaves its unclaimed slots uncached. The normal
        // on-demand path will retry them without failing the build.
        for worker in workers {
            let _ = worker.join();
        }
    });

    result_slots
        .into_iter()
        .map(|slot| slot.into_inner().unwrap_or(OpenResult::NotAvailable))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefetch_concurrency_is_bounded() {
        assert_eq!(prefetch_concurrency(None, 100), 16);
        assert_eq!(prefetch_concurrency(Some("4"), 100), 4);
        assert_eq!(prefetch_concurrency(Some("0"), 100), 16);
        assert_eq!(prefetch_concurrency(Some("invalid"), 100), 16);
        assert_eq!(prefetch_concurrency(Some("1000"), 1000), 64);
        assert_eq!(prefetch_concurrency(Some("1000"), 3), 3);
        assert_eq!(prefetch_concurrency(Some("4"), 0), 0);
    }

    #[test]
    fn prefetch_selection_caps_total_bytes() {
        let mib = 1024 * 1024;
        let infos = [
            ItarFileInfo {
                name: "first".into(),
                offset: 0,
                length: 40 * mib,
            },
            ItarFileInfo {
                name: "too-large-for-remainder".into(),
                offset: 0,
                length: 30 * mib,
            },
            ItarFileInfo {
                name: "fits".into(),
                offset: 0,
                length: 20 * mib,
            },
        ];

        assert_eq!(prefetch_indices(&infos), vec![0, 2]);
    }
}
