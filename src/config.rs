// src/config.rs -- configuration for the Tectonic library.
// Copyright 2016-2018 the Tectonic Project
// Licensed under the MIT License.

//! User configuration settings for the Tectonic engine.
//!
//! Because Tectonic has a goal of having a high level of reproducibility, we
//! aim to *avoid* persistent configuration options as much as possible. But,
//! we at least need a mechanism for specifying the default bundle to use when
//! running the command-line client. So we begrudgingly have a *little*
//! configuration.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use app_dirs::{app_dir, sanitized, AppDataType};

use errors::{ErrorKind, Result};
use io::itarbundle::{HttpITarIoFactory, ITarBundle};
use io::local_cache::LocalCache;
use io::zipbundle::ZipBundle;
use io::Bundle;
use status::StatusBackend;

/// Awesome hack time!!!
///
/// This is part of the "test mode" described in the `test_util` module. When
/// test mode is activated in this module, the `default_bundle()` and
/// `format_cache_path()` functions return results pointing to the test asset
/// tree, rather than whatever the user has actually configured.
static CONFIG_TEST_MODE_ACTIVATED: AtomicBool = AtomicBool::new(false);

#[doc(hidden)]
pub fn activate_config_test_mode(forced: bool) {
    CONFIG_TEST_MODE_ACTIVATED.store(forced, Ordering::SeqCst);
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PersistentConfig {
    default_bundles: Vec<BundleInfo>,
}

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BundleInfo {
    url: String,
}

impl PersistentConfig {
    #[cfg(feature = "serialization")]
    /// Open the per-user configuration file.
    ///
    /// This file is stored in TOML format. If the configuration file does not
    /// exist, no error is signaled — instead, a basic default configuration
    /// is returned. In this case, if `auto_create_config_file` is true, the
    /// configuration file (and the directory containing it) will be
    /// automatically created, filling in the default configuration. If it is
    /// false, the default configuration is returned and the filesystem is not
    /// modified.
    pub fn open(auto_create_config_file: bool) -> Result<PersistentConfig> {
        use app_dirs::{app_root, get_app_root};
        use std::io::ErrorKind as IoErrorKind;
        use std::io::{Read, Write};
        use toml;
        let mut cfg_path = if auto_create_config_file {
            app_root(AppDataType::UserConfig, &::APP_INFO)?
        } else {
            get_app_root(AppDataType::UserConfig, &::APP_INFO)?
        };
        cfg_path.push("config.toml");

        let config = match File::open(&cfg_path) {
            Ok(mut f) => {
                let mut buf = Vec::<u8>::new();
                f.read_to_end(&mut buf)?;
                toml::from_slice(&buf)?
            }
            Err(e) => {
                if e.kind() == IoErrorKind::NotFound {
                    // Config file didn't exist -- that's OK.
                    let config = PersistentConfig::default();
                    if auto_create_config_file {
                        let mut f = File::create(&cfg_path)?;
                        write!(f, "{}", toml::to_string(&config)?)?;
                    }
                    config
                } else {
                    // Uh oh, unexpected error reading the config file.
                    return Err(e.into());
                }
            }
        };

        Ok(config)
    }

    #[cfg(not(feature = "serialization"))]
    /// Return a default configuration structure.
    ///
    /// In most builds of Tectonic, this function reads a per-user
    /// configuration file and returns it. However, this version of Tectonic
    /// has been built without the `serde` feature, so it cannot deserialize
    /// the file. Therefore, this function always returns the default
    /// configuration.
    pub fn open(_auto_create_config_file: bool) -> Result<PersistentConfig> {
        Ok(PersistentConfig::default())
    }

    pub fn make_cached_url_provider(
        &self,
        url: &str,
        only_cached: bool,
        status: &mut StatusBackend,
    ) -> Result<Box<Bundle>> {
        let itb = ITarBundle::<HttpITarIoFactory>::new(url);

        let mut url2digest_path = app_dir(AppDataType::UserCache, &::APP_INFO, "urls")?;
        url2digest_path.push(sanitized(url));

        let bundle = LocalCache::<ITarBundle<HttpITarIoFactory>>::new(
            itb,
            &url2digest_path,
            &app_dir(AppDataType::UserCache, &::APP_INFO, "manifests")?,
            &app_dir(AppDataType::UserCache, &::APP_INFO, "files")?,
            only_cached,
            status,
        )?;

        Ok(Box::new(bundle) as _)
    }

    pub fn make_local_file_provider(
        &self,
        file_path: &OsStr,
        _status: &mut StatusBackend,
    ) -> Result<Box<Bundle>> {
        use std::path::Path;

        let zip_bundle = ZipBundle::<File>::open(Path::new(file_path))?;

        Ok(Box::new(zip_bundle) as _)
    }

    pub fn default_bundle(
        &self,
        only_cached: bool,
        status: &mut StatusBackend,
    ) -> Result<Box<Bundle>> {
        use hyper::Url;
        use std::io;

        if CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst) {
            return Ok(Box::new(::test_util::TestBundle::default()));
        }

        if self.default_bundles.len() != 1 {
            return Err(ErrorKind::Msg(
                "exactly one default_bundle item must be specified (for now)".to_owned(),
            )
            .into());
        }

        let url = Url::parse(&self.default_bundles[0].url)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "failed to parse url"))?;
        if url.scheme() == "file" {
            // load the local zip file.
            let file_path = url.to_file_path().map_err(|_| {
                io::Error::new(io::ErrorKind::InvalidInput, "failed to parse local path")
            })?;
            let zip_bundle = self.make_local_file_provider(file_path.as_os_str(), status)?;

            return Ok(Box::new(zip_bundle) as _);
        }
        let bundle =
            self.make_cached_url_provider(&self.default_bundles[0].url, only_cached, status)?;
        Ok(Box::new(bundle) as _)
    }

    pub fn format_cache_path(&self) -> Result<PathBuf> {
        if CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst) {
            Ok(::test_util::test_path(&[]))
        } else {
            Ok(app_dir(AppDataType::UserCache, &::APP_INFO, "formats")?)
        }
    }
}

impl Default for PersistentConfig {
    fn default() -> Self {
        PersistentConfig {
            default_bundles: vec![BundleInfo {
                url: String::from("https://archive.org/services/purl/net/pkgwpub/tectonic-default"),
            }],
        }
    }
}
