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

use std::io::{Read, Write};
use std::io::ErrorKind as IoErrorKind;
use std::fs::File;
use std::path::PathBuf;
use std::ffi::OsStr;

use app_dirs::{app_dir, app_root, get_app_root, sanitized, AppDataType};
use toml;

use errors::{ErrorKind, Result};
use io::itarbundle::{HttpITarIoFactory, ITarBundle};
use io::local_cache::LocalCache;
use io::zipbundle::ZipBundle;
use io::Bundle;
use status::StatusBackend;


const DEFAULT_CONFIG: &'static str = r#"[[default_bundles]]
url = "https://purl.org/net/pkgwpub/tectonic-default"
"#;


#[derive(Deserialize)]
pub struct PersistentConfig {
    default_bundles: Vec<BundleInfo>,
}

#[derive(Deserialize)]
pub struct BundleInfo {
    url: String,
}

impl PersistentConfig {
    pub fn open(auto_create_config_file: bool) -> Result<PersistentConfig> {
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
            },
            Err(e) => {
                if e.kind() == IoErrorKind::NotFound {
                    // Config file didn't exist -- that's OK.
                    if auto_create_config_file {
                        let mut f = File::create(&cfg_path)?;
                        write!(f, "{}", DEFAULT_CONFIG)?;
                    }
                    toml::from_str(DEFAULT_CONFIG)?
                } else {
                    // Uh oh, unexpected error reading the config file.
                    return Err(e.into());
                }
            },
        };

        Ok(config)
    }

    pub fn make_cached_url_provider(&self, url: &str, only_cached: bool, status: &mut StatusBackend) -> Result<Box<Bundle>> {
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

    pub fn default_bundle(&self, only_cached: bool, status: &mut StatusBackend) -> Result<Box<Bundle>> {
        use std::io;
        use hyper::Url;

        if self.default_bundles.len() != 1 {
            return Err(ErrorKind::Msg("exactly one default_bundle item must be specified (for now)".to_owned()).into());
        }

        let url = Url::parse(&self.default_bundles[0].url)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "failed to parse url"))?;
        if url.scheme() == "file" {
            // load the local zip file.
            let file_path = url.to_file_path()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "failed to parse local path"))?;
            let zip_bundle = self.make_local_file_provider(file_path.as_os_str(), status)?;

            return Ok(Box::new(zip_bundle) as _);
        }
        let bundle = self.make_cached_url_provider(&self.default_bundles[0].url, only_cached, status)?;
    		return Ok(Box::new(bundle) as _);
    }

    pub fn format_cache_path(&self) -> Result<PathBuf> {
        Ok(app_dir(AppDataType::UserCache, &::APP_INFO, "formats")?)
    }
}


impl Default for PersistentConfig {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG).expect("un-parseable built-in default configuration (?!)")
    }
}
