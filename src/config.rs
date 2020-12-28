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
use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};

use crate::app_dirs;
use crate::errors::{ErrorKind, Result};
use crate::io::cached_itarbundle::CachedITarBundle;
use crate::io::dirbundle::DirBundle;
use crate::io::zipbundle::ZipBundle;
use crate::io::Bundle;
use crate::status::StatusBackend;

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

#[doc(hidden)]
pub fn is_config_test_mode_activated() -> bool {
    CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst)
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
        use std::io::ErrorKind as IoErrorKind;
        use std::io::{Read, Write};
        let mut cfg_path = if auto_create_config_file {
            app_dirs::user_config()?
        } else {
            app_dirs::get_user_config()?
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
        custom_cache_root: Option<&Path>,
        status: &mut dyn StatusBackend,
    ) -> Result<Box<dyn Bundle>> {
        let bundle = CachedITarBundle::new(url, only_cached, custom_cache_root, status)?;

        Ok(Box::new(bundle) as _)
    }

    pub fn make_local_file_provider(
        &self,
        file_path: PathBuf,
        _status: &mut dyn StatusBackend,
    ) -> Result<Box<dyn Bundle>> {
        let bundle: Box<dyn Bundle> = if file_path.is_dir() {
            Box::new(DirBundle::new(file_path))
        } else {
            Box::new(ZipBundle::open(file_path)?)
        };
        Ok(bundle)
    }

    pub fn default_bundle_loc(&self) -> &str {
        &self.default_bundles[0].url
    }

    pub fn default_bundle(
        &self,
        only_cached: bool,
        status: &mut dyn StatusBackend,
    ) -> Result<Box<dyn Bundle>> {
        use reqwest::Url;
        use std::io;

        if CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst) {
            return Ok(Box::new(crate::test_util::TestBundle::default()));
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
            return self.make_local_file_provider(file_path, status);
        }
        let bundle =
            self.make_cached_url_provider(&self.default_bundles[0].url, only_cached, None, status)?;
        Ok(Box::new(bundle) as _)
    }

    pub fn format_cache_path(&self) -> Result<PathBuf> {
        if CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst) {
            Ok(crate::test_util::test_path(&[]))
        } else {
            Ok(app_dirs::user_cache_dir("formats")?)
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
