use anyhow::{bail, Result};
use reqwest::header;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::command::snowflake;
use crate::{env, github::TokenOk};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SnowflakeAuthenticationMethod<'a> {
    Password { password: &'a str },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateSourceParameters<'a> {
    Snowflake {
        organization: snowflake::OrganizationName,
        account: &'a str,
        database: &'a str,
        user: &'a str,
        authentication_method: SnowflakeAuthenticationMethod<'a>,
        warehouse: &'a str,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GetSourceParameters {
    Snowflake {
        organization: snowflake::OrganizationName,
        account: String,
        database: String,
        user: String,
        warehouse: String,
    },
}

#[derive(Debug, Serialize)]
pub struct CreateSourceInput<'a> {
    pub name: &'a str,
    pub source_parameters: CreateSourceParameters<'a>,
}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(session: &TokenOk) -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        let mut auth_value =
            header::HeaderValue::from_str(&format!("Bearer {}", &session.access_token)).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);

        Ok(Client {
            client: reqwest::Client::builder()
                .user_agent(env::user_agent())
                .default_headers(headers)
                .build()?,
        })
    }

    pub async fn create_source(&self, input: &CreateSourceInput<'_>) -> Result<()> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().push("sources");

        let response = self.client.post(url).json(&input).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{}, body: {}", status, body);
        }

        Ok(())
    }

    pub async fn list_sources(&self) -> Result<ListSourcesResponse> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().push("sources");

        let response = self.client.get(url).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{}, body: {}", status, body);
        }
        Ok(serde_json::from_str(&body)?)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Source {
    id: Uuid,
    name: String,
    source_parameters: GetSourceParameters,
}

#[derive(Deserialize)]
pub struct ListSourcesResponse {
    pub sources: Vec<Source>,
}
