use anyhow::{bail, Result};
use reqwest::header;
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::command::snowflake;
use crate::descriptor::{DataResource, Name};
use crate::env;

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
    pub fn new(token: &str) -> Result<Client> {
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap();
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

        let response = self.client.post(url.clone()).json(&input).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }

        Ok(())
    }

    pub async fn get_source(&self, name: &str) -> Result<GetSourceResponse> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().push("sources").push(name);

        let response = self.client.get(url.clone()).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn list_sources(&self) -> Result<ListSourcesResponse> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().push("sources");

        let response = self.client.get(url.clone()).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }
        Ok(serde_json::from_str(&body)?)
    }

    /// Creates a version of a package (and package itself, if it doesn't yet exist).
    pub async fn create_version(
        &self,
        package_id: uuid7::Uuid,
        version: &Version,
        input: &CreatePackageVersion<'_>,
    ) -> Result<()> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().extend(&[
            "packages",
            &package_id.to_string(),
            "versions",
            &version.to_string(),
        ]);

        let response = self.client.put(url.clone()).json(&input).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }
        Ok(())
    }

    pub async fn get_package_version(
        &self,
        name: &str,
        version: semver::Version,
    ) -> Result<GetPackageVersionResponse> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().push("packages");
        url.path_segments_mut().unwrap().push(name);
        url.path_segments_mut().unwrap().push("version");
        url.path_segments_mut().unwrap().push(&version.to_string());

        let response = self.client.get(url).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{}, body: {}", status, body);
        }
        Ok(serde_json::from_str(&body)?)
    }
}

type GetSourceResponse = Source;

#[derive(Deserialize, Serialize)]
pub struct Source {
    #[serde(rename = "uuid")]
    pub id: Uuid,
    pub name: String,
    pub source_parameters: GetSourceParameters,
}

impl Source {
    pub fn type_name(&self) -> String {
        match self.source_parameters {
            GetSourceParameters::Snowflake { .. } => "snowflake".into(),
        }
    }
}

#[derive(Deserialize)]
pub struct ListSourcesResponse {
    pub sources: Vec<Source>,
}

#[derive(Serialize)]
pub struct CreatePackageVersion<'a> {
    /// Identifier for the package to create a version for.
    pub name: &'a Name,
    /// The package description as of this version.
    pub description: &'a String,
    pub dataset: &'a Vec<DataResource>,
}

#[derive(Deserialize, Serialize)]
pub struct Constraints {
    maxlength: Option<i64>,
    required: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Field {
    constraints: Constraints,
    format: String,
    name: String,
    r#type: String,
    bare_number: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct DatasetSchema {
    missing_values: Vec<String>, // rename this
    fields: Vec<Field>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourcePath {
    Snowflake { schema: String, table: String },
}

#[derive(Deserialize, Serialize)]
pub struct DatasetSource {
    id: Uuid,
    path: SourcePath,
}

#[derive(Deserialize, Serialize)]
pub struct Dataset {
    pub description: String,
    pub schema: DatasetSchema,
    pub name: String,
    pub source: DatasetSource,
}

#[derive(Deserialize)]
pub struct GetPackageVersionResponse {
    pub name: String,
    pub uuid: Uuid,
    pub description: Option<String>,
    pub version_major: i64,
    pub version_minor: i64,
    pub version_patch: i64,
    pub dataset: Vec<Dataset>,
}
