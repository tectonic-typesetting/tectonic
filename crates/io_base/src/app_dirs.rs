// Copyright 2019-2021 the Tectonic Project
// Licensed under the MIT License.

//! Default directories for per-user Tectonic files.
//!
//! If you want to use your own directories for your own application, that's OK,
//! but if you want to look at Tectonic’s default configuration and/or cache
//! data, these are the places to go.

use directories::ProjectDirs;
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{env, fs, io};
use tectonic_errors::prelude::*;

/// The instance of the `directories` crate that this crate links to.
pub use directories;

static PROJECT_DIRS: LazyLock<Option<ProjectDirs>> =
    LazyLock::new(|| ProjectDirs::from("", "TectonicProject", "Tectonic"));

fn dirs() -> Result<&'static ProjectDirs> {
    PROJECT_DIRS.as_ref().ok_or_else(|| {
        Error::from(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to find standard directories for platform",
        ))
    })
}

/// Get the directory for per-user Tectonic configuration files.
///
/// This constructs the path but does not ensure that the directory actually
/// exists. The function [`ensure_user_config`] makes sure that the directory is
/// created.
///
/// This function is currently implemented with [`ProjectDirs::config_dir`] using
/// the `ProjectDirs` data type. Return values have the form:
///
/// - Windows: `%APPDATA%\TectonicProject\Tectonic`, where `%APPDATA%` is
///   something like `C:\Users\knuth\AppData\Roaming`.
/// - macOS: `$HOME/Library/Application Support/Tectonic`
/// - Others: `$XDG_CONFIG_HOME/Tectonic` if defined, otherwise
///   `$HOME/.config/Tectonic`
pub fn get_user_config() -> Result<PathBuf> {
    Ok(dirs()?.config_dir().to_path_buf())
}

/// Get the directory for per-user Tectonic configuration files, creating it if needed.
///
/// This is largely the same as [`get_user_config`], but ensures that the
/// returned directory actually exists.
pub fn ensure_user_config() -> Result<PathBuf> {
    let path = get_user_config()?;
    fs::create_dir_all(&path)?;
    Ok(path)
}

/// Get a directory for per-user Tectonic cache files, creating it if needed.
///
/// The *path* argument may include subdirectories, but the directory separator
/// should be a forward slash on all platforms. It may be an empty string if you
/// want to get the toplevel user cache directory.
///
/// This function is currently implemented with [`ProjectDirs::cache_dir`] using the
/// `ProjectDirs` data type. Return values have the form:
///
/// - Windows: `%LOCALAPPDATA%\TectonicProject\Tectonic`, where `%LOCALAPPDATA%`
///   is something like `C:\Users\knuth\AppData\Local`.
/// - macOS: `$HOME/Library/Caches/Tectonic`
/// - Others: `$XDG_CACHE_HOME/Tectonic` if defined, otherwise
///   `$HOME/.cache/Tectonic`
///
///
/// The cache location defaults to the `ProjectDirs::cache_dir`
/// provided by `directories` but can be overwritten using the
/// `TECTONIC_CACHE_DIR` environment variable.
///
/// This method may perform I/O to create the user cache directory, so it is
/// fallible. (Due to its `directories` implementation, it would have to be
/// fallible even if it didn't perform I/O.)
pub fn get_user_cache_dir(subdir: &str) -> Result<PathBuf> {
    let env_cache_path = env::var_os("TECTONIC_CACHE_DIR");

    let cache_path = match env_cache_path {
        Some(env_cache_path) => {
            let mut env_cache_path: PathBuf = env_cache_path.into();
            env_cache_path.push(subdir);
            fs::create_dir_all(&env_cache_path)?;
            env_cache_path
        }
        None => dirs()?.cache_dir().join(subdir),
    };

    Ok(cache_path)
}

/// Borrowed from `app_dirs2`, convert any string into a consistently valid file or directory name.
pub fn sanitize(component: &str) -> String {
    let mut buf = String::with_capacity(component.len());
    for (i, c) in component.chars().enumerate() {
        let is_alnum = c.is_ascii_alphanumeric();
        let is_space = c == ' ';
        let is_hyphen = c == '-';
        let is_underscore = c == '_';
        let is_period = c == '.' && i != 0; // Disallow accidentally hidden folders
        let is_valid = is_alnum || is_space || is_hyphen || is_underscore || is_period;
        if is_valid {
            buf.push(c);
        } else {
            use std::fmt::Write;
            let _ = write!(&mut buf, ",{},", c as u32);
        }
    }
    buf
}
