use crate::env;
use crate::github;

/// Ensures that a valid session is stored in the CLI's session.json file.
///
/// The CLI authenticates its requests to the DPM Cloud API by presenting a
/// bearer token with the request: a [user access
/// token](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app)
/// issued to the DPM Cloud GitHub App. Then this token is stored on the filesystem,
/// in a location where running data packages can easily find it.
pub async fn login() -> Result<(), String> {
    let token = github::login().await?;
    println!();

    let config_path = env::ensure_config_dir()?;
    let session_path = config_path.join("session.json");
    let contents = serde_json::to_string_pretty(&token).map_err(|e| e.to_string())?;
    std::fs::write(&session_path, contents)
        .map_err(|e| format!("error writing session file: {}", e))?;

    println!(
        "Session file written to: {}",
        session_path.to_string_lossy()
    );
    println!("You are now logged in.");
    Ok(())
}
