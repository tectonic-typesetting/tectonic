// src/config.rs -- configuration for the Tectonic library.
// Copyright 2016-2017 the Tectonic Project
// Licensed under the MIT License.

/// Because Tectonic has a goal of having a high level of reproducibility, we
/// aim to *avoid* configuration options as much as possible. But, we at least
/// need a mechanism for specifying the default bundle to use when running the
/// command-line client. So we begrudgingly have a *little* configuration.

use std::io::{Read, Write};
use std::io::ErrorKind as IoErrorKind;
use std::fs::File;

use app_dirs::{app_dir, app_root, get_app_root, sanitized, AppDataType};
use toml;

use errors::{ErrorKind, Result};
use io::IoProvider;
use io::itarbundle::{HttpITarIoFactory, ITarBundle};
use io::local_cache::LocalCache;
use status::StatusBackend;


const DEFAULT_CONFIG: &'static str = r#"[[default_bundles]]
url = "http://purl.org/net/pkgwpub/tectonic-default"
"#;


#[derive(Deserialize)]
pub struct Config {
    default_bundles: Vec<BundleInfo>,
}

#[derive(Deserialize)]
pub struct BundleInfo {
    url: String,
}


impl Config {
    pub fn open(auto_create_config_file: bool) -> Result<Config> {
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

    fn make_cached_url_provider(&self, url: &str, status: &mut StatusBackend) -> Result<LocalCache<ITarBundle<HttpITarIoFactory>>> {
        let itb = ITarBundle::<HttpITarIoFactory>::new(url);

        let mut url2digest_path = app_dir(AppDataType::UserCache, &::APP_INFO, "urls")?;
        url2digest_path.push(sanitized(url));

        LocalCache::<ITarBundle<HttpITarIoFactory>>::new(
            itb,
            &url2digest_path,
            &app_dir(AppDataType::UserCache, &::APP_INFO, "manifests")?,
            &app_dir(AppDataType::UserCache, &::APP_INFO, "files")?,
            status
        )
    }

    pub fn default_io_provider(&self, status: &mut StatusBackend) -> Result<Box<IoProvider>> {
        if self.default_bundles.len() != 1 {
            return Err(ErrorKind::Msg("exactly one default_bundle item must be specified (for now)".to_owned()).into());
        }

        Ok(Box::new(self.make_cached_url_provider(&self.default_bundles[0].url, status)?))
    }
}
