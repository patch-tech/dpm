use std::fmt::Display;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use reqwest::{header, StatusCode};
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::command::snowflake;
use crate::descriptor::{Name, Table, TableSource};
use crate::env;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SnowflakeAuthenticationMethod<'a> {
    Password { password: &'a str },
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CreateSourceParameters<'a> {
    #[serde(rename = "bigquery")]
    BigQuery {
        project_id: &'a str,
        dataset: &'a str,
        staging_project_id: &'a str,
        #[serde(rename = "credentials_key")]
        service_account_key_b64: String,
    },
    Snowflake {
        organization: snowflake::OrganizationName,
        account: &'a str,
        database: &'a str,
        user: &'a str,
        authentication_method: SnowflakeAuthenticationMethod<'a>,
        staging_database: &'a str,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum GetSourceParameters {
    #[serde(rename = "bigquery")]
    BigQuery {
        project_id: String,
        dataset: String,
        staging_project_id: String,
    },
    Snowflake {
        organization: snowflake::OrganizationName,
        account: String,
        database: String,
        user: String,
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

    pub async fn get_source_metadata(&self, id: Uuid) -> Result<GetSourceMetadataResponse> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut()
            .unwrap()
            .extend(&["sources", &id.to_string(), "metadata"]);

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

    /// Creates a version of a dataset (and the dataset itself, if it doesn't
    /// yet exist).
    pub async fn create_version(
        &self,
        dataset_id: uuid7::Uuid,
        version: &Version,
        input: &CreateDatasetVersion<'_>,
    ) -> Result<DatasetVersion> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().extend(&[
            "packages",
            &dataset_id.to_string(),
            "versions",
            &version.to_string(),
        ]);

        let response = self.client.put(url.clone()).json(&input).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }

        // The response actual has quite a bit of data on it; deserializing it
        // into a DatasetVersion only captures a subset of it, but it's a
        // sufficient subset for the current callers of
        // `Client::create_version`.
        Ok(serde_json::from_str(&body)?)
    }

    pub async fn list_datasets(&self) -> Result<ListDatasetsResponse> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut().unwrap().extend(&["packages"]);

        let response = self.client.get(url.clone()).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }

        Ok(serde_json::from_str(&body)?)
    }

    /// Returns all versions (draft and release) in reverse version order,
    /// or `None` if the dataset doesn't exist or isn't readable.
    pub async fn get_dataset_versions(
        &self,
        identifier: &str,
    ) -> Result<Option<GetDatasetResponse>> {
        let mut url = env::api_base_url()?;
        url.path_segments_mut()
            .unwrap()
            .extend(&["packages", identifier]);

        let response = self.client.get(url.clone()).send().await?;
        let status = response.status();
        let body = response.text().await?;
        if status == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !status.is_success() {
            bail!("{} => {}, body: {}", url, status, body);
        }

        let mut response = serde_json::from_str::<GetDatasetResponse>(&body)?;
        response
            .dataset_versions
            .sort_unstable_by(|a, b| b.version.cmp(&a.version));

        Ok(Some(response))
    }

    pub async fn get_dataset_version(
        &self,
        name: &str,
        version: semver::Version,
    ) -> Result<Option<GetDatasetVersionResponse>> {
        let response = match self.get_dataset_versions(name).await? {
            Some(response) => response,
            None => return Ok(None),
        };

        Ok(Some(GetDatasetVersionResponse {
            package_uuid: response.uuid,
            package_name: response.name,
            package_description: response.description,
            version: response
                .dataset_versions
                .into_iter()
                .find(|p| p.version == version)
                .with_context(|| format!("no such version published: {}", version))?,
        }))
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
            GetSourceParameters::BigQuery { .. } => "bigquery".into(),
            GetSourceParameters::Snowflake { .. } => "snowflake".into(),
        }
    }
}

/// Known variants of config-api's `TableColumnDescription.dpmBetaType`. This
/// type defines which operators one may apply to a field at query time.
///
/// Note that the above is a distinct concern from two related concerns:
/// - How a value of type T is represented in a given serialization format (say, JSON)
/// - How a value of type T is represented when returned to a caller who's using
///   a data package
///
/// In general, the maximum precision for any given type is undefined.
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DpmBetaType {
    /// Sequence of Unicode code points.
    String,
    /// True or false.
    Boolean,
    /// [Decimal fraction](https://en.wikipedia.org/wiki/Decimal#Decimal_fractions).
    Number,
    /// Date in the proleptic Gregorian calendar.
    Date,
    /// Triple of hours, minutes, and seconds.
    Time,
    /// A combination of `Date` and `Time`, referring to an instant on the UTC
    /// time scale.
    DateTime,
    /// A list of values. May or may not be homogeneous.
    Array,
}

impl FromStr for DpmBetaType {
    type Err = serde_json::Error;

    /// Parses a value from strings like "string", "boolean", etc.
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        serde_json::from_value(serde_json::Value::String(s.to_owned()))
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldSchema {
    pub name: String,
    /// If `None`, the field is not yet supported by any dpm query interface.
    pub dpm_beta_type: Option<String>,
    pub nullable: bool,
    pub source_type: String,
}

#[derive(Deserialize)]
pub struct TableSchema {
    pub fields: Vec<FieldSchema>,
}

#[derive(Deserialize)]
pub struct TableMetadata {
    pub schema: TableSchema,
    pub source: TableSource,
}

#[derive(Deserialize)]
pub struct GetSourceMetadataResponse {
    pub metadata: Vec<TableMetadata>,
}

#[derive(Deserialize)]
pub struct ListSourcesResponse {
    pub sources: Vec<Source>,
}

#[derive(Deserialize, Serialize)]
pub struct ListDatasetsResponse {
    #[serde(rename = "packages")]
    pub datasets: Vec<GetDatasetResponse>,
}

#[derive(Serialize)]
pub struct CreateDatasetVersion<'a> {
    /// Identifier for the dataset to create a version for.
    pub name: &'a Name,
    /// Whether this version is a draft or (if not) a release.
    pub draft: bool,
    /// Whether this version should be accelerated by Patch.
    #[serde(rename = "patch_accelerated")]
    pub accelerated: bool,
    /// The dataset description as of this version.
    pub description: &'a String,
    #[serde(rename = "dataset")]
    pub tables: &'a Vec<Table>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum PatchState {
    SyncingInitial,
    ErrorSyncingInitial,
    Syncing,
    ErrorSyncing,
}

impl Display for PatchState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            PatchState::SyncingInitial => "Syncing initial",
            PatchState::ErrorSyncingInitial => "Error syncing initial",
            PatchState::Syncing => "Healthy",
            PatchState::ErrorSyncing => "Error syncing",
        })
    }
}

#[derive(Deserialize, Serialize)]
pub struct DatasetVersion {
    pub version: Version,
    #[serde(default)]
    pub accelerated: bool,
    #[serde(rename(deserialize = "patch_state", serialize = "acceleration_state"))]
    pub patch_state: Option<PatchState>,
    pub patch_state_data: Option<serde_json::Value>,
    pub dataset: Vec<Table>,
}

pub struct GetDatasetVersionResponse {
    pub package_name: String,
    pub package_uuid: Uuid,
    pub package_description: String,
    pub version: DatasetVersion,
}

#[derive(Deserialize, Serialize)]
pub struct GetDatasetResponse {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    #[serde(rename = "package_versions")]
    pub dataset_versions: Vec<DatasetVersion>,
}
