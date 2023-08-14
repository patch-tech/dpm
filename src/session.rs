use anyhow::{Context, Result};

use crate::{env, github::TokenOk};

/// Reads the filesystem and returns the current session, if it exists.
fn get_maybe_invalid() -> Result<Option<TokenOk>> {
    let path = env::session_path()?;
    if !path.try_exists().with_context(|| {
        format!(
            "Existence check failed for session file: {}",
            path.display()
        )
    })? {
        return Ok(None);
    }
    let file = std::fs::File::open(&path)
        .with_context(|| format!("Failed to open session file: {}", path.display()))?;
    let session = serde_json::from_reader(file)
        .with_context(|| format!("Failed to deserialize session file: {}", path.display()))?;
    Ok(Some(session))
}

/// Returns the stored `TokenOk`, or an `Err` advising the user to log in.
fn get() -> Result<TokenOk> {
    get_maybe_invalid()?.context("Not logged in. (tip: Log in with `dpm login`)")
}

/// Returns a DPM Cloud API token. Precedence order:
///   1. DPM_AUTH_TOKEN environment variable
///   2. session.json file, created via `dpm login`
pub fn get_token() -> Result<String> {
    if let Ok(t) = std::env::var("DPM_AUTH_TOKEN") {
        return Ok(t);
    }

    Ok(get()?.access_token)
}
