//! Functions to discover information about the environment.

use std::{fs, path::PathBuf};

use directories::ProjectDirs;

/// Returns the path to the CLI's configuration directory, which surely exists.
pub fn ensure_config_dir() -> Result<PathBuf, String> {
    let project_dir = ProjectDirs::from("tech", "patch", "dpm")
        .ok_or("could not locate project directory".to_string())?;

    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir).map_err(|err| err.to_string())?;

    Ok(config_dir.to_path_buf())
}
