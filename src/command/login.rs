use anyhow::{Context, Result};

use crate::env;
use crate::github;
use crate::session;

/// Ensures that a valid session is stored in the CLI's session.json file.
///
/// The CLI authenticates its requests to the DPM Cloud API by presenting a
/// bearer token with the request: a [user access
/// token](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
/// issued to the Patch GitHub App. Then this token is stored on the filesystem,
/// in a location where running data packages can easily find it.
pub async fn login() -> Result<()> {
    let session_path = env::session_path()?;
    if session_path.exists() {
        let token = session::get_token()?;
        if github::token_is_valid(&token).await? {
            println!("Session file present at: {}", session_path.display());
            println!("You are already logged in.");
            return Ok(());
        }
        // If token exists but doesn't pass cursory validation, proceed with the
        // login flow as though the invalid session.json weren't present at all.
    }

    let token = github::login().await?;
    println!();

    let contents = serde_json::to_string_pretty(&token)?;
    std::fs::write(&session_path, contents)
        .with_context(|| format!("Error writing session file: {}", session_path.display()))?;

    println!("Session file written to: {}", session_path.display());
    println!("You are now logged in.");
    Ok(())
}
