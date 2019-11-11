// Copyright 2019 the Tectonic Project
// Licensed under the MIT License.

use crate::errors::Result;
use app_dirs::AppDataType;
use std::path::PathBuf;

pub use app_dirs::sanitized;

const APP_INFO: app_dirs::AppInfo = app_dirs::AppInfo {
    name: "Tectonic",
    author: "TectonicProject",
};

#[cfg(feature = "serialization")]
pub fn user_config() -> Result<PathBuf> {
    Ok(app_dirs::app_root(AppDataType::UserConfig, &APP_INFO)?)
}

#[cfg(feature = "serialization")]
pub fn get_user_config() -> Result<PathBuf> {
    Ok(app_dirs::get_app_root(AppDataType::UserConfig, &APP_INFO)?)
}

pub fn user_cache_dir(path: &str) -> Result<PathBuf> {
    Ok(app_dirs::app_dir(AppDataType::UserCache, &APP_INFO, path)?)
}
