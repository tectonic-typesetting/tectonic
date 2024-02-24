// Copyright 2016-2021 the Tectonic Project
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
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};
use tectonic_bundles::{
    cache::Cache, dir::DirBundle, itar::IndexedTarBackend, zip::ZipBundle, Bundle,
};
use tectonic_io_base::app_dirs;
use url::Url;

use crate::errors::{ErrorKind, Result};

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

pub fn is_test_bundle_wanted(web_bundle: Option<String>) -> bool {
    if !is_config_test_mode_activated() {
        return false;
    }
    match web_bundle {
        None => true,
        Some(x) if x.contains("test-bundle://") => true,
        _ => false,
    }
}

pub fn maybe_return_test_bundle(web_bundle: Option<String>) -> Result<Box<dyn Bundle>> {
    if is_test_bundle_wanted(web_bundle) {
        Ok(Box::<crate::test_util::TestBundle>::default())
    } else {
        Err(ErrorKind::Msg("not asking for the default test bundle".to_owned()).into())
    }
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
        use std::{
            fs::File,
            io::{ErrorKind as IoErrorKind, Read, Write},
        };

        let mut cfg_path = if auto_create_config_file {
            app_dirs::ensure_user_config()?
        } else {
            app_dirs::get_user_config()?
        };
        cfg_path.push("config.toml");

        let config = match File::open(&cfg_path) {
            Ok(mut f) => {
                let mut buf = String::new();
                f.read_to_string(&mut buf)?;
                toml::from_str(&buf)?
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
    ) -> Result<Box<dyn Bundle>> {
        if let Ok(test_bundle) = maybe_return_test_bundle(Some(url.to_owned())) {
            return Ok(test_bundle);
        }

        let mut cache = if let Some(root) = custom_cache_root {
            Cache::get_for_custom_directory(root)
        } else {
            Cache::get_user_default()?
        };

        let bundle = cache.open::<IndexedTarBackend>(url, only_cached)?;
        Ok(Box::new(bundle) as _)
    }

    pub fn make_local_file_provider(&self, file_path: PathBuf) -> Result<Box<dyn Bundle>> {
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

    pub fn default_bundle(&self, only_cached: bool) -> Result<Box<dyn Bundle>> {
        use std::io;

        if let Ok(test_bundle) = maybe_return_test_bundle(None) {
            return Ok(test_bundle);
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
            return self.make_local_file_provider(file_path);
        }
        let bundle =
            self.make_cached_url_provider(&self.default_bundles[0].url, only_cached, None)?;
        Ok(Box::new(bundle) as _)
    }

    pub fn format_cache_path(&self) -> Result<PathBuf> {
        if is_config_test_mode_activated() {
            Ok(crate::test_util::test_path(&[]))
        } else {
            Ok(app_dirs::ensure_user_cache_dir("formats")?)
        }
    }
}

impl Default for PersistentConfig {
    fn default() -> Self {
        let url = tectonic_bundles::get_fallback_bundle_url(tectonic_engine_xetex::FORMAT_SERIAL);

        PersistentConfig {
            default_bundles: vec![BundleInfo { url }],
        }
    }
}
