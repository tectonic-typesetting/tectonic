// Copyright 2026 the Tectonic Project
// Licensed under the MIT License.

//! Selection and caching of Biber executables for CLI document builds.

use anyhow::{bail, Context};
use flate2::read::GzDecoder;
use quick_xml::{events::Event, Reader};
use sha2::Digest;
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};
use tectonic_errors::Result;
use tectonic_geturl::{DefaultBackend, GetUrlBackend};
use tectonic_io_base::app_dirs;

const MAX_ARCHIVE_BYTES: u64 = 200 * 1024 * 1024;

#[derive(Clone, Copy)]
enum ArchiveKind {
    TarGz,
    Zip,
}

/// A checked upstream archive for a particular Biber version and platform.
struct Artifact<'a> {
    version: &'a str,
    platform: &'a str,
    url: &'a str,
    archive_sha256: &'a str,
    archive_kind: ArchiveKind,
}

const ARTIFACTS: &[Artifact<'static>] = &[
    Artifact {
        version: "2.17",
        platform: "linux-x86_64-gnu",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.17/binaries/Linux/biber-linux_x86_64.tar.gz",
        archive_sha256: "129d2e0332a57e985ffa253e5e9fbd28ef99af5a068d1b141145211969aa8999",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.17",
        platform: "windows-x86_64",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.17/binaries/Windows/biber-MSWIN64.zip",
        archive_sha256: "c103bffc5ae0a7f513e7c26b6d394e9be6cf41952959c5d604ee2e6581b5dea2",
        archive_kind: ArchiveKind::Zip,
    },
    Artifact {
        version: "2.17",
        platform: "macos-universal",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.17/binaries/MacOS/biber-darwin_universal.tar.gz",
        archive_sha256: "182e1efa074d8a2a23a8893f2a22440d4e463cce55e4ed02076ac4c0ee0614b2",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.18",
        platform: "linux-x86_64-gnu",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.18/binaries/Linux/biber-linux_x86_64.tar.gz",
        archive_sha256: "2a6b4cd15a1139907799da0d23cd4ddcce8341af3960d2b3d1d3e4b4a9f1fb53",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.18",
        platform: "windows-x86_64",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.18/binaries/Windows/biber-MSWIN64.zip",
        archive_sha256: "02ee3a8b6838b7ff1e9aea1a5342686981b9364067a7b8e7131c3b3201cf387c",
        archive_kind: ArchiveKind::Zip,
    },
    Artifact {
        version: "2.18",
        platform: "macos-universal",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.18/binaries/MacOS/biber-darwin_universal.tar.gz",
        archive_sha256: "a0848ca266334284f1145470e53c3882b2d5e8ad82828700df1f6e347f7b675b",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.19",
        platform: "linux-x86_64-gnu",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.19/binaries/Linux/biber-linux_x86_64.tar.gz",
        archive_sha256: "e2eda3e6ea7ac7e78d60e99a0e2aeb1096829f95791c06b768ed31a12889e58e",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.19",
        platform: "windows-x86_64",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.19/binaries/Windows/biber-MSWIN64.zip",
        archive_sha256: "f0bccdec320e89a04b067f1189957b4bbe6feb445005357601f6e295e83e97da",
        archive_kind: ArchiveKind::Zip,
    },
    Artifact {
        version: "2.19",
        platform: "macos-universal",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.19/binaries/MacOS/biber-darwin_universal.tar.gz",
        archive_sha256: "0ebda145064eb5b8901a4ed5c8c5e5e6a5208e0aba425f7febcb5fb5b1a9c11b",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.20",
        platform: "linux-x86_64-gnu",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.20/binaries/Linux/biber-linux_x86_64.tar.gz",
        archive_sha256: "50ec098c1d0813c5b5dcae109fe3dcf7403a0d8a5fb0235775d7eae5cf24c5b6",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.20",
        platform: "windows-x86_64",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.20/binaries/Windows/biber-MSWIN64.zip",
        archive_sha256: "b35d28f9e641b3b69c8fe2ab60518da6dc55907feaf4b9a018304446f591ca2f",
        archive_kind: ArchiveKind::Zip,
    },
    Artifact {
        version: "2.20",
        platform: "macos-universal",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.20/binaries/MacOS/biber-darwin_universal.tar.gz",
        archive_sha256: "441561b5ec503e16283278650403de23aa7bb6ae3397504e3a7e3292151b4757",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.21",
        platform: "linux-x86_64-gnu",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.21/binaries/Linux/biber-linux_x86_64.tar.gz",
        archive_sha256: "f00dfa29c7f798695339d9155abefcc0da4bd2fb1b4b2d90e46693f261b0a26e",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.21",
        platform: "windows-x86_64",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.21/binaries/Windows/biber-MSWIN64.zip",
        archive_sha256: "2ae8323193db40f87f6471b3c9a8378059bf73a37ce15189f788770e6d93e353",
        archive_kind: ArchiveKind::Zip,
    },
    Artifact {
        version: "2.21",
        platform: "macos-universal",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/2.21/binaries/MacOS/biber-darwin_universal.tar.gz",
        archive_sha256: "8c895defed5e69b7a824cb7b7947e8bbfa3f3b17ffb8a1d493e982b679e6633c",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.22",
        platform: "linux-x86_64-gnu",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/current/binaries/Linux/biber-linux_x86_64.tar.gz",
        archive_sha256: "92d77e668aac41a45842846a1dfb3b857d42cbc4eadf45f4d23c338aeaf89734",
        archive_kind: ArchiveKind::TarGz,
    },
    Artifact {
        version: "2.22",
        platform: "windows-x86_64",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/current/binaries/Windows/biber-MSWIN64.zip",
        archive_sha256: "b3be34ac97b34aae93a4bd46788a71c80e9dc803c2601cfe211d423b417f40f9",
        archive_kind: ArchiveKind::Zip,
    },
    Artifact {
        version: "2.22",
        platform: "macos-universal",
        url: "https://downloads.sourceforge.net/project/biblatex-biber/biblatex-biber/current/binaries/MacOS/biber-darwin_universal.tar.gz",
        archive_sha256: "b0dc9804c803057a65f8dd5399f4c10e8dde207678dc9806a90a9d232afa0f37",
        archive_kind: ArchiveKind::TarGz,
    },
];

fn platform() -> Option<&'static str> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") if !cfg!(target_env = "musl") => Some("linux-x86_64-gnu"),
        ("windows", "x86_64") => Some("windows-x86_64"),
        ("macos", "x86_64" | "aarch64") => Some("macos-universal"),
        _ => None,
    }
}

fn artifact_for(version: &str) -> Option<&'static Artifact<'static>> {
    let platform = platform()?;
    ARTIFACTS
        .iter()
        .find(|artifact| artifact.version == version && artifact.platform == platform)
}

fn sha256_file(path: &Path) -> Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(tectonic_io_base::digest::bytes_to_hex(&hasher.finalize()))
}

fn verified_cache_path(root: &Path, artifact: &Artifact<'_>) -> Result<Option<PathBuf>> {
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("biber-") || !name.ends_with(".sha256") {
            continue;
        }
        let saved = fs::read_to_string(entry.path())?;
        let Some((archive_hash, executable_hash)) = saved.trim().split_once(' ') else {
            continue;
        };
        if archive_hash != artifact.archive_sha256 || executable_hash.len() != 64 {
            continue;
        }
        let executable_name = name.trim_end_matches(".sha256");
        let path = root.join(executable_name);
        match sha256_file(&path) {
            Ok(hash) if hash == executable_hash => return Ok(Some(path)),
            Ok(_) => {}
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|io_error| io_error.kind() == std::io::ErrorKind::NotFound) => {}
            Err(error) => return Err(error),
        }
    }
    Ok(None)
}

fn extract_executable(
    archive_path: &Path,
    artifact: &Artifact<'_>,
    output: &mut File,
) -> Result<()> {
    match artifact.archive_kind {
        ArchiveKind::TarGz => {
            let archive_file = File::open(archive_path)?;
            let mut archive = tar::Archive::new(GzDecoder::new(archive_file));
            let mut found = false;
            for entry in archive.entries()? {
                let mut entry = entry?;
                let path = entry.path()?;
                if (path.as_ref() == Path::new("biber") || path.as_ref() == Path::new("./biber"))
                    && entry.header().entry_type().is_file()
                {
                    if found {
                        bail!("Biber archive contains multiple executable entries");
                    }
                    std::io::copy(&mut entry, output)?;
                    found = true;
                }
            }
            if !found {
                bail!("Biber archive does not contain its executable");
            }
        }
        ArchiveKind::Zip => {
            let archive_file = File::open(archive_path)?;
            let mut archive = zip::ZipArchive::new(archive_file)?;
            let mut entry = archive.by_name("biber.exe")?;
            std::io::copy(&mut entry, output)?;
        }
    }
    Ok(())
}

fn resolve_with_backend<B: GetUrlBackend>(
    artifact: &Artifact<'_>,
    root: &Path,
    only_cached: bool,
    backend: &mut B,
) -> Result<PathBuf> {
    if let Some(path) = verified_cache_path(root, artifact)? {
        return Ok(path);
    }
    if only_cached {
        bail!(
            "compatible Biber {} is not in Tectonic's cache and --only-cached prevents downloading it",
            artifact.version
        );
    }

    fs::create_dir_all(root)?;
    let archive = download_verified_archive(artifact, root, backend)?;

    let mut executable = tempfile::Builder::new()
        .prefix("biber-")
        .suffix(if cfg!(windows) { ".exe" } else { "" })
        .tempfile_in(root)?;
    extract_executable(archive.path(), artifact, executable.as_file_mut())?;
    executable.flush()?;
    let executable_hash = sha256_file(executable.path())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        executable
            .as_file()
            .set_permissions(fs::Permissions::from_mode(0o700))?;
    }
    let (_, final_path) = executable.keep()?;
    let mut marker = tempfile::NamedTempFile::new_in(root)?;
    write!(marker, "{} {}", artifact.archive_sha256, executable_hash)?;
    let marker_path = root.join(format!(
        "{}.sha256",
        final_path.file_name().unwrap().to_string_lossy()
    ));
    marker.persist(marker_path)?;
    Ok(final_path)
}

fn download_verified_archive<B: GetUrlBackend>(
    artifact: &Artifact<'_>,
    root: &Path,
    backend: &mut B,
) -> Result<tempfile::NamedTempFile> {
    let mut urls = vec![artifact.url.to_owned()];
    // SourceForge moves the newest release from `current` into its numbered
    // directory when the next version arrives. The checksum stays pinned.
    if artifact.version == "2.22" {
        urls.push(artifact.url.replace("/current/", "/2.22/"));
    }

    let mut last_error = None;
    for url in urls {
        let attempt = (|| -> Result<tempfile::NamedTempFile> {
            let mut archive = tempfile::NamedTempFile::new_in(root)?;
            let response = backend.get_url(&url).with_context(|| {
                format!("could not download Biber {} from {url}", artifact.version)
            })?;
            let bytes = std::io::copy(
                &mut response.take(MAX_ARCHIVE_BYTES + 1),
                archive.as_file_mut(),
            )?;
            if bytes > MAX_ARCHIVE_BYTES {
                bail!("Biber archive from {url} exceeds the maximum allowed size");
            }
            archive.flush()?;
            if sha256_file(archive.path())? != artifact.archive_sha256 {
                bail!("downloaded Biber archive from {url} failed SHA-256 verification");
            }
            Ok(archive)
        })();
        match attempt {
            Ok(archive) => return Ok(archive),
            Err(error) => last_error = Some(error),
        }
    }
    Err(last_error.unwrap())
}

/// Resolve an upstream Biber binary if a vetted artifact exists for this host.
pub(crate) fn resolve(version: &str, only_cached: bool) -> Result<Option<PathBuf>> {
    let Some(artifact) = artifact_for(version) else {
        return Ok(None);
    };
    let tools_dir = app_dirs::get_user_cache_dir("tools")?;
    let tools_dir = if tools_dir.is_absolute() {
        tools_dir
    } else {
        std::env::current_dir()?.join(tools_dir)
    };
    let root = tools_dir
        .join("biber")
        .join(artifact.version)
        .join(artifact.platform);
    resolve_with_backend(artifact, &root, only_cached, &mut DefaultBackend::default()).map(Some)
}

/// Check an already-installed Biber executable without running it on a document.
pub(crate) fn installed_version(path: &Path) -> Result<Option<String>> {
    let output = Command::new(path).arg("--version").output()?;
    if !output.status.success() {
        return Ok(None);
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let version = text
        .split_whitespace()
        .find(|word| word.starts_with("2."))
        .map(|word| {
            word.trim_end_matches(|ch: char| !ch.is_ascii_digit())
                .to_owned()
        });
    Ok(version)
}

/// Read the biblatex version from a control file generated by biblatex.
pub(crate) fn biblatex_version(bcf: &[u8]) -> Option<String> {
    let mut reader = Reader::from_reader(bcf);
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf).ok()? {
            Event::Start(element) | Event::Empty(element)
                if element.local_name().as_ref() == b"controlfile" =>
            {
                for attribute in element.attributes().flatten() {
                    if attribute.key.as_ref() == b"bltxversion" {
                        return attribute
                            .decode_and_unescape_value(reader.decoder())
                            .ok()
                            .map(|value| value.into_owned());
                    }
                }
                return None;
            }
            Event::Eof => return None,
            _ => {}
        }
        buf.clear();
    }
}

/// Versions of Biber for which upstream publishes compatible biblatex releases.
pub(crate) fn compatible_biber_version(biblatex: &str) -> Option<&'static str> {
    let minor = biblatex.strip_prefix("3.")?;
    let minor = minor.trim_end_matches(|ch: char| ch.is_ascii_alphabetic());
    match minor {
        "17" => Some("2.17"),
        "18" => Some("2.18"),
        "19" => Some("2.19"),
        "20" => Some("2.20"),
        "21" => Some("2.21"),
        "22" => Some("2.22"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        biblatex_version, compatible_biber_version, resolve_with_backend, ArchiveKind, Artifact,
    };
    use anyhow::bail;
    use flate2::{write::GzEncoder, Compression};
    use sha2::{Digest, Sha256};
    use std::io::Cursor;
    use tectonic_errors::Result;
    use tectonic_geturl::{GetUrlBackend, RangeReader};

    #[derive(Default)]
    struct FixtureBackend {
        archive: Vec<u8>,
        archived_release: Option<Vec<u8>>,
        requests: usize,
    }

    struct UnusedRangeReader;

    impl RangeReader for UnusedRangeReader {
        type Response = Cursor<Vec<u8>>;

        fn read_range(&mut self, _offset: u64, _length: usize) -> Result<Self::Response> {
            bail!("range requests are not used for Biber archives")
        }
    }

    impl GetUrlBackend for FixtureBackend {
        type Response = Cursor<Vec<u8>>;
        type RangeReader = UnusedRangeReader;

        fn resolve_url(&mut self, url: &str) -> Result<String> {
            Ok(url.to_owned())
        }

        fn get_url(&mut self, url: &str) -> Result<Self::Response> {
            self.requests += 1;
            if url.contains("/2.22/") {
                if let Some(archive) = &self.archived_release {
                    return Ok(Cursor::new(archive.clone()));
                }
            }
            Ok(Cursor::new(self.archive.clone()))
        }

        fn open_range_reader(&self, _url: &str) -> Self::RangeReader {
            UnusedRangeReader
        }
    }

    fn fixture_archive(contents: &[u8]) -> Vec<u8> {
        let encoder = GzEncoder::new(Vec::new(), Compression::default());
        let mut builder = tar::Builder::new(encoder);
        let mut header = tar::Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o755);
        header.set_cksum();
        builder.append_data(&mut header, "biber", contents).unwrap();
        builder.into_inner().unwrap().finish().unwrap()
    }

    fn fixture_artifact<'a>(hash: &'a str) -> Artifact<'a> {
        Artifact {
            version: "2.17",
            platform: "fixture",
            url: "https://example.invalid/biber.tar.gz",
            archive_sha256: hash,
            archive_kind: ArchiveKind::TarGz,
        }
    }

    #[test]
    fn zip_archive_installs_only_executable() {
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        writer
            .start_file("biber.exe", zip::write::SimpleFileOptions::default())
            .unwrap();
        std::io::Write::write_all(&mut writer, b"fixture biber").unwrap();
        let archive = writer.finish().unwrap().into_inner();
        let hash = tectonic_io_base::digest::bytes_to_hex(&Sha256::digest(&archive));
        let artifact = Artifact {
            archive_kind: ArchiveKind::Zip,
            ..fixture_artifact(&hash)
        };
        let root = tempfile::tempdir().unwrap();
        let mut backend = FixtureBackend {
            archive,
            archived_release: None,
            requests: 0,
        };

        let path = resolve_with_backend(&artifact, root.path(), false, &mut backend).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), b"fixture biber");
    }

    #[test]
    fn current_release_moves_to_numbered_directory() {
        let archive = fixture_archive(b"fixture biber");
        let hash = tectonic_io_base::digest::bytes_to_hex(&Sha256::digest(&archive));
        let artifact = Artifact {
            version: "2.22",
            url: "https://example.invalid/current/biber.tar.gz",
            ..fixture_artifact(&hash)
        };
        let root = tempfile::tempdir().unwrap();
        let mut backend = FixtureBackend {
            archive: fixture_archive(b"newer release"),
            archived_release: Some(archive),
            requests: 0,
        };

        let path = resolve_with_backend(&artifact, root.path(), false, &mut backend).unwrap();
        assert_eq!(std::fs::read(path).unwrap(), b"fixture biber");
        assert_eq!(backend.requests, 2);
    }

    #[test]
    fn reads_biblatex_version_from_control_file() {
        let bcf = br#"<?xml version="1.0"?><bcf:controlfile xmlns:bcf="https://example.com" version="3.8" bltxversion="3.17"/>"#;
        assert_eq!(biblatex_version(bcf).as_deref(), Some("3.17"));
        assert_eq!(biblatex_version(b"not XML"), None);
    }

    #[test]
    fn selects_matching_biber_release() {
        assert_eq!(compatible_biber_version("3.17"), Some("2.17"));
        assert_eq!(compatible_biber_version("3.22a"), Some("2.22"));
        assert_eq!(compatible_biber_version("3.23"), None);
    }

    #[test]
    fn download_is_verified_and_reused_offline() {
        let archive = fixture_archive(b"fixture biber");
        let hash = tectonic_io_base::digest::bytes_to_hex(&Sha256::digest(&archive));
        let artifact = fixture_artifact(&hash);
        let root = tempfile::tempdir().unwrap();
        let mut backend = FixtureBackend {
            archive,
            archived_release: None,
            requests: 0,
        };

        let path = resolve_with_backend(&artifact, root.path(), false, &mut backend).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), b"fixture biber");
        assert_eq!(backend.requests, 1);
        assert!(resolve_with_backend(&artifact, root.path(), true, &mut backend).is_ok());
        assert_eq!(backend.requests, 1);

        std::fs::write(&path, b"corrupt").unwrap();
        assert!(resolve_with_backend(&artifact, root.path(), true, &mut backend).is_err());
        let repaired = resolve_with_backend(&artifact, root.path(), false, &mut backend).unwrap();
        assert_ne!(path, repaired);
        assert!(path.exists());
        assert_eq!(std::fs::read(repaired).unwrap(), b"fixture biber");
        assert_eq!(backend.requests, 2);
    }

    #[test]
    fn offline_miss_and_corrupt_archive_do_not_install_executable() {
        let root = tempfile::tempdir().unwrap();
        let wrong_hash = "0".repeat(64);
        let artifact = fixture_artifact(&wrong_hash);
        let mut backend = FixtureBackend {
            archive: fixture_archive(b"fixture biber"),
            archived_release: None,
            requests: 0,
        };
        assert!(resolve_with_backend(&artifact, root.path(), true, &mut backend).is_err());
        assert_eq!(backend.requests, 0);
        assert!(resolve_with_backend(&artifact, root.path(), false, &mut backend).is_err());
        assert_eq!(backend.requests, 1);
        assert!(std::fs::read_dir(root.path()).unwrap().next().is_none());
    }
}
