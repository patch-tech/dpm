use anyhow::{bail, Context, Result};
use copypasta::{ClipboardContext, ClipboardProvider};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;
use url::Url;

use crate::env;

/// Client ID of the DPM Cloud GitHub App.
const GITHUB_APP_CLIENT_ID: &str = "Iv1.3dc84c4afac087ff";

/// Executes an OAuth 2.0 Device Authorization Grant against GitHub, obtaining a
/// user access token issued to the DPM Cloud GitHub App.
///
/// References:
/// - https://oauth.net/2/device-flow/
/// - https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app#using-the-device-flow-to-generate-a-user-access-token
/// - https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/refreshing-user-access-tokens
/// - https://docs.github.com/en/apps/creating-github-apps/writing-code-for-a-github-app/building-a-cli-with-a-github-app
pub async fn login() -> Result<TokenOk> {
    // 1. POST https://github.com/login/device/code
    let client = reqwest::Client::new();
    let res = client
        .post("https://github.com/login/device/code")
        .query(&[("client_id", GITHUB_APP_CLIENT_ID)])
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await?
        .json::<DeviceAuthorizationResponse>()
        .await
        .context("Deserializing GitHub device authorization response")?;

    // 2. Copy code to clipboard and prompt user to open URL
    enum Clipboard {
        /// Clipboard access is unsupported, and/or there's no string on the
        /// clipboard currently.
        Unmanaged,
        /// Clipboard access is supported and there exists a string already on
        /// the clipboard.
        Managed(ClipboardContext, String),
    }
    let mut clipboard = match ClipboardContext::new() {
        Ok(mut ctx) => match ctx.get_contents() {
            Ok(current) => Clipboard::Managed(ctx, current),
            Err(_) => Clipboard::Unmanaged,
        },
        Err(_) => Clipboard::Unmanaged,
    };

    if let Clipboard::Managed(ref mut ctx, _) = clipboard {
        ctx.set_contents(res.user_code.to_owned())
            .expect("error setting clipboard contents");
        eprintln!(
            "Login code has been copied to clipboard. Or, copy it here: {}",
            res.user_code
        );
    } else {
        eprintln!("Copy the following login code: {}", res.user_code)
    }

    eprintln!(
        "Enter it at this URL: {}",
        res.verification_uri_complete
            .unwrap_or(res.verification_uri)
    );

    // 3. Start polling POST https://github.com/login/oauth/access_token.
    let token = poll_for_token(&res.device_code, res.interval).await;

    // 4. Restore clipboard contents
    if let Clipboard::Managed(ref mut ctx, previous_clipboard_string) = clipboard {
        match ctx.set_contents(previous_clipboard_string.clone()) {
            Ok(_) => eprintln!("Clipboard contents restored"),
            Err(_) => eprintln!("Failed to restore clipboard contents"),
        }
    }

    token
}

/// Polls the GitHub token endpoint until a token is obtained or until a
/// terminal error occurs. Returns the token or an error message.
async fn poll_for_token(device_code: &str, initial_interval: Duration) -> Result<TokenOk> {
    let mut interval = initial_interval;

    loop {
        let token_response = request_token(device_code).await;

        // This `match` decides which token endpoint responses are terminal and
        // which mean we should continue polling the token endpoint.
        match token_response {
            Ok(token_ok) => return Ok(token_ok),
            Err(TokenErr::Oauth(OauthTokenErr {
                error,
                error_description,
                ..
            })) => match error {
                TokenErrCode::AuthorizationPending => (/* do nothing, keep polling */),
                TokenErrCode::AccessDenied => {
                    bail!(error_description.unwrap_or("authorization request denied".into()))
                }
                TokenErrCode::ExpiredToken => {
                    bail!(error_description.unwrap_or("grant request expired".into()))
                }
                TokenErrCode::SlowDown => {
                    interval += Duration::from_secs(5);
                }
                TokenErrCode::InvalidRequest
                | TokenErrCode::InvalidClient
                | TokenErrCode::InvalidGrant
                | TokenErrCode::UnauthorizedClient
                | TokenErrCode::UnsupportedGrantType
                | TokenErrCode::InvalidScope
                | TokenErrCode::IncorrectClientCredentials
                | TokenErrCode::IncorrectDeviceCode
                | TokenErrCode::DeviceFlowDisabled => {
                    bail!("unexpected error (please log an issue!): {:?}", error)
                }
            },
            Err(TokenErr::Io(e)) => return Err(anyhow::Error::from(e)).context("I/O error"),
            Err(TokenErr::Github(s)) => bail!("GitHub API error: {}", s),
        };

        sleep(interval).await;
    }
}

/// See https://datatracker.ietf.org/doc/html/rfc8628#section-3.2
#[derive(Debug, Deserialize)]
struct DeviceAuthorizationResponse {
    device_code: String,
    user_code: String,
    verification_uri: Url,
    verification_uri_complete: Option<Url>,
    #[serde(with = "duration_serde")]
    #[allow(dead_code)]
    expires_in: Duration,
    // Optional in OAuth, but required in GitHub's implementation.
    #[serde(with = "duration_serde")]
    interval: Duration,
}

mod duration_serde {
    //! Durations from GitHub are JSON numbers representing seconds

    use std::time::Duration;

    use serde::{Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = serde_json::Number::deserialize(deserializer)?;

        if num.is_u64() {
            Ok(Duration::from_secs(num.as_u64().unwrap()))
        } else if num.is_f64() {
            Duration::try_from_secs_f64(num.as_f64().unwrap()).map_err(serde::de::Error::custom)
        } else {
            Err(serde::de::Error::custom(
                "duration was neither a u64 nor f64",
            ))
        }
    }

    #[allow(dead_code)]
    pub fn serialize<S>(v: &Duration, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(v.as_secs())
    }
}

/// A "Device Access Token Request" success response, which is defined as
/// equal to the general token endpoint success response.
///
/// Refs:
/// - https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
/// - https://datatracker.ietf.org/doc/html/rfc6749#section-5.1
#[derive(Deserialize, Serialize)]
pub struct TokenOk {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<u32>,
    pub scope: Option<String>,
}

/// Anything that can go wrong during a request to the GitHub token endpoint
/// during a device authorization grant.
enum TokenErr {
    Oauth(OauthTokenErr),
    Github(String),
    Io(reqwest::Error),
}

/// A GitHub "Device Access Token Request" error response, which is equal to the
/// generic OAuth token endpoint error response, but with a larger set of
/// possible error codes.
#[derive(Deserialize)]
struct OauthTokenErr {
    error: TokenErrCode,
    error_description: Option<String>,
    #[allow(dead_code)]
    error_uri: Url,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TokenErrCode {
    // General OAuth token endpoint errors.
    // https://datatracker.ietf.org/doc/html/rfc6749#section-5.2
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,

    // Errors unique to device authorization grants.
    // https://datatracker.ietf.org/doc/html/rfc8628#section-3.5
    AuthorizationPending,
    SlowDown,
    AccessDenied,
    ExpiredToken,

    // Errors unique to GitHub device authorization grants.
    // https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-a-user-access-token-for-a-github-app#using-the-device-flow-to-generate-a-user-access-token
    IncorrectClientCredentials,
    IncorrectDeviceCode,
    DeviceFlowDisabled,
}

/// Redeems a device code with GitHub's token endpoint.
async fn request_token(device_code: &str) -> Result<TokenOk, TokenErr> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://github.com/login/oauth/access_token")
        .query(&[
            ("client_id", GITHUB_APP_CLIENT_ID),
            ("device_code", device_code),
            ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ])
        .header(reqwest::header::ACCEPT, "application/json")
        .send()
        .await
        .map_err(TokenErr::Io)?;

    let text = response.text().await.map_err(TokenErr::Io)?;
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) else {
        return Err(TokenErr::Github(format!(
            "response from GitHub token endpoint was not a JSON object: {}",
            text
        )));
    };
    let Some(map) = value.as_object() else {
        return Err(TokenErr::Github(format!(
            "response from GitHub token endpoint was not a JSON object: {}",
            text
        )));
    };

    if map.contains_key("access_token") {
        Ok(serde_json::from_value(value).map_err(|e| {
            TokenErr::Github(format!(
                "success response from GitHub token endpoint could not be deserialized: {}",
                e
            ))
        })?)
    } else if map.contains_key("error") {
        Err(TokenErr::Oauth(serde_json::from_value(value).map_err(
            |e| {
                TokenErr::Github(format!(
                    "error response from GitHub token endpoint could not be deserialized: {}",
                    e
                ))
            },
        )?))
    } else {
        return Err(TokenErr::Github(format!(
            "response from GitHub token endpoint was neither a success nor error object: {}",
            text
        )));
    }
}

pub async fn token_is_valid(token: &str) -> Result<bool> {
    let client = reqwest::Client::new();
    let res = client
        .get("https://api.github.com/user")
        .header(reqwest::header::ACCEPT, "application/vnd.github+json")
        .header(reqwest::header::USER_AGENT, env::user_agent())
        .bearer_auth(token)
        .send()
        .await?;

    match res.status() {
        reqwest::StatusCode::OK => Ok(true),
        reqwest::StatusCode::UNAUTHORIZED => Ok(false),
        status => {
            let body = res.text().await?;
            bail!(
                "unexpected response from GitHub during token validation: {}, {}",
                status,
                body
            )
        }
    }
}
