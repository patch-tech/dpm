//! Functions to discover information about the environment.

use anyhow::{Context, Result};

use std::{fs, path::PathBuf};

use directories::ProjectDirs;

use crate::built_info;

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

/// Value to use for any User-Agent HTTP request headers sent from this
/// application.
pub fn user_agent() -> String {
    let mut git_sha = built_info::GIT_COMMIT_HASH_SHORT.unwrap().to_string();
    if built_info::GIT_DIRTY.unwrap_or(false) {
        git_sha.push_str("-dirty");
    }

    format!("dpm/{} ({git_sha})", built_info::PKG_VERSION)
}
