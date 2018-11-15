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
use std::ffi::OsStr;
use std::fs::File;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::result;
use std::fmt;

use app_dirs::{app_dir, app_root, get_app_root, sanitized, AppDataType};
use toml;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

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


const DEFAULT_CONFIG: &'static str = r#"[[default_bundles]]
url = "https://archive.org/services/purl/net/pkgwpub/tectonic-default"
"#;


pub struct PersistentConfig {
    default_bundles: Vec<BundleInfo>,
}

// Manual implementation of Deserialize because serde_derive does not work with musl
// See https://github.com/rust-lang/rust/issues/40174
// Implementation based on https://serde.rs/deserialize-struct.html
impl<'de> Deserialize<'de> for PersistentConfig {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { DefaultBundles };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> result::Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`default_bundles`")
                    }

                    fn visit_str<E>(self, value: &str) -> result::Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "default_bundles" => Ok(Field::DefaultBundles),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PersistentConfigVisitor;

        impl<'de> Visitor<'de> for PersistentConfigVisitor {
            type Value = PersistentConfig;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct PersistentConfig")
            }

            fn visit_map<V>(self, mut map: V) -> result::Result<PersistentConfig, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut default_bundles = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::DefaultBundles => {
                            if default_bundles.is_some() {
                                return Err(de::Error::duplicate_field("default_bundles"));
                            }
                            default_bundles = Some(map.next_value()?);
                        }
                    }
                }
                let default_bundles = default_bundles.ok_or_else(|| de::Error::missing_field("default_bundles"))?;
                Ok(PersistentConfig { default_bundles })
            }
        }

        const FIELDS: &'static [&'static str] = &["default_bundles"];
        deserializer.deserialize_struct("PersistentConfig", FIELDS, PersistentConfigVisitor)
    }
}

pub struct BundleInfo {
    url: String,
}

impl<'de> Deserialize<'de> for BundleInfo {
    fn deserialize<D>(deserializer: D) -> result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { Url };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> result::Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`url`")
                    }

                    fn visit_str<E>(self, value: &str) -> result::Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "url" => Ok(Field::Url),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct BundleInfoVisitor;

        impl<'de> Visitor<'de> for BundleInfoVisitor {
            type Value = BundleInfo;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct BundleInfo")
            }

            fn visit_map<V>(self, mut map: V) -> result::Result<BundleInfo, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut url = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Url => {
                            if url.is_some() {
                                return Err(de::Error::duplicate_field("url"));
                            }
                            url = Some(map.next_value()?);
                        }
                    }
                }
                let url = url.ok_or_else(|| de::Error::missing_field("url"))?;
                Ok(BundleInfo { url })
            }
        }

        const FIELDS: &'static [&'static str] = &["url"];
        deserializer.deserialize_struct("BundleInfo", FIELDS, BundleInfoVisitor)
    }
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

        if CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst) {
            return Ok(Box::new(::test_util::TestBundle::default()));
        }

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
        if CONFIG_TEST_MODE_ACTIVATED.load(Ordering::SeqCst) {
            return Ok(::test_util::test_path(&[]));
        } else {
            Ok(app_dir(AppDataType::UserCache, &::APP_INFO, "formats")?)
        }
    }
}


impl Default for PersistentConfig {
    fn default() -> Self {
        toml::from_str(DEFAULT_CONFIG).expect("un-parseable built-in default configuration (?!)")
    }
}
