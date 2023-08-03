//! Functions to discover information about the environment.

use anyhow::{Context, Result};

use std::{fs, path::PathBuf};

use directories::ProjectDirs;

/// Returns the path to the CLI's configuration directory, which surely exists.
pub fn ensure_config_dir() -> Result<PathBuf> {
    let project_dir =
        ProjectDirs::from("tech", "patch", "dpm").context("could not locate project directory")?;

    let config_dir = project_dir.config_dir();
    fs::create_dir_all(config_dir)?;

    Ok(config_dir.to_path_buf())
}

pub fn session_path() -> Result<PathBuf> {
    Ok(ensure_config_dir()?.join("session.json"))
}
