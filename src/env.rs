//! Functions to discover information about the environment.

use anyhow::{bail, Context, Result};
use url::Url;

use std::{
    env::{self, VarError},
    fs,
    path::PathBuf,
};

use directories::ProjectDirs;

use crate::built_info;

pub fn api_base_url() -> Result<Url> {
    let s = match env::var("DPM_API_URL") {
        Ok(v) => v,
        Err(VarError::NotPresent) => "https://api.dpm.sh".into(),
        Err(VarError::NotUnicode(_)) => bail!("DPM_API_URL is not Unicode"),
    };
    Url::parse(&s).map_err(Into::into)
}

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
    let git_sha = built_info::GIT_COMMIT_HASH_SHORT;
    match git_sha {
        Some(git_sha) => {
            let mut git_sha = git_sha.to_string();
            if built_info::GIT_DIRTY.unwrap_or(false) {
                git_sha.push_str("-dirty");
            }
            format!("dpm/{} ({git_sha})", built_info::PKG_VERSION)
        }
        None => format!("dpm/{}", built_info::PKG_VERSION),
    }
}

/// Whether this code is running as part of a CI job.
fn is_ci() -> bool {
    // This is always set in GitHub Actions, e.g.
    // See https://docs.github.com/en/actions/learn-github-actions/variables#default-environment-variables
    std::env::var("CI").map_or(false, |s| s == "true")
}

/// Whether this code is running during a test run.
pub fn is_test() -> bool {
    is_ci() || std::env::var("TEST").map_or(false, |s| s == "true")
}
