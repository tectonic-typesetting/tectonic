// Copyright 2019-2021 the Tectonic Project
// Licensed under the MIT License.

//! Default directories for per-user Tectonic files.
//!
//! If you want to use your own directories for your own application, that's OK,
//! but if you want to look at Tectonic’s default configuration and/or cache
//! data, these are the places to go.

use app_dirs2::AppDataType;
use std::path::PathBuf;
use tectonic_errors::prelude::*;

/// The instance of the `app_dirs2` crate that this crate links to.
pub use app_dirs2;

/// Maybe we should just make this public? But we preserve some flexibility by
/// not doing so just yet.
const APP_INFO: app_dirs2::AppInfo = app_dirs2::AppInfo {
    name: "Tectonic",
    author: "TectonicProject",
};

/// Get the directory for per-user Tectonic configuration files.
///
/// This constructs the path but does not ensure that the directory actually
/// exists. The function [`ensure_user_config`] makes sure that the directory is
/// created.
///
/// This function is currently implemented with [`app_dirs2::get_app_root`] using
/// the `UserConfig` data type. Return values have the form:
///
/// - Windows: `%APPDATA%\TectonicProject\Tectonic`, where `%APPDATA%` is
///   something like `C:\Users\knuth\AppData\Roaming`.
/// - macOS: `$HOME/Library/Application Support/Tectonic`
/// - Others: `$XDG_CONFIG_HOME/Tectonic` if defined, otherwise
///   `$HOME/.config/Tectonic`
pub fn get_user_config() -> Result<PathBuf> {
    Ok(app_dirs2::get_app_root(AppDataType::UserConfig, &APP_INFO)?)
}

/// Get the directory for per-user Tectonic configuration files, creating it if needed.
///
/// This is largely the same as [`get_user_config`], but ensures that the
/// returned directory actually exists.
pub fn ensure_user_config() -> Result<PathBuf> {
    Ok(app_dirs2::app_root(AppDataType::UserConfig, &APP_INFO)?)
}

/// Get a directory for per-user Tectonic cache files, creating it if needed.
///
/// The *path* argument may include subdirectories, but the directory separator
/// should be a forward slash on all platforms. It may be an empty string if you
/// want to get the toplevel user cache directory.
///
/// This function is currently implemented with [`app_dirs2::app_dir`] using the
/// `UserCache` data type. Return values have the form:
///
/// - Windows: `%LOCALAPPDATA%\TectonicProject\Tectonic`, where `%LOCALAPPDATA%`
///   is something like `C:\Users\knuth\AppData\Local`.
/// - macOS: `$HOME/Library/Caches/Tectonic`
/// - Others: `$XDG_CACHE_HOME/Tectonic` if defined, otherwise
///   `$HOME/.cache/Tectonic`
pub fn ensure_user_cache_dir(path: &str) -> Result<PathBuf> {
    Ok(app_dirs2::app_dir(AppDataType::UserCache, &APP_INFO, path)?)
}
