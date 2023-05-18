use std::env;
use std::path::PathBuf;

/// Obtain the executable name without a prefix if the executable is available in the PATH, e.g.
/// most cases. Otherwise, use the full path e.g. in development.
pub(crate) fn get_trimmed_exe_name() -> PathBuf {
    let exe_name = env::current_exe().expect("Get current executable name");

    let path = env::var("PATH").unwrap_or_else(|_| env::var("Path").unwrap_or_default());
    let paths = env::split_paths(&path).collect::<Vec<_>>();

    for path in paths {
        if let Ok(p) = exe_name.strip_prefix(&path) {
            return p.to_owned();
        }
    }
    exe_name
}
