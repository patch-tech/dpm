use std::path::PathBuf;

use anyhow::Result;
use base64::{engine::general_purpose::STANDARD as b64, Engine};
use clap::Subcommand;

use crate::{
    api::{Client, CreateSourceInput, CreateSourceParameters, SnowflakeAuthenticationMethod},
    session,
};

use super::snowflake;

#[derive(Debug, Subcommand)]
pub enum CreateSource {
    /// Create a BigQuery source
    #[command(name = "bigquery")]
    BigQuery {
        /// Name to give the created source.
        #[arg(long, short)]
        name: String,

        /// ID of your Google Cloud project.
        #[arg(long)]
        project_id: String,

        /// ID of the Google Cloud project which dpm will use to perform change
        /// data capture on tables in this source. This value is only used when
        /// there exist accelerated data packages that access data from this
        /// source.
        #[arg(long)]
        staging_project_id: String,

        /// Path to a JSON file containing a GCP service account key. For
        /// instructions on how to create such a file, see
        /// https://cloud.google.com/iam/docs/keys-create-delete#creating.
        #[arg(long, value_name = "PATH")]
        credentials_key: PathBuf,
    },
    /// Create a Snowflake source
    Snowflake {
        /// Name to give the created source.
        #[arg(long, short)]
        name: String,

        #[arg(long, value_name = "NAME")]
        organization: snowflake::OrganizationName,

        #[arg(long, value_name = "NAME")]
        account: String,

        #[arg(long, value_name = "NAME")]
        database: String,

        #[arg(long, value_name = "NAME")]
        user: String,

        #[arg(long)]
        password: String,

        /// Database which dpm Cloud will use to perform change data capture on
        /// tables in this source. This value is only used when there exist
        /// accelerated data packages that access data from this source. For
        /// more information, see
        /// https://docs.dpm.sh/sources/snowflake#1-provisioning.
        #[arg(long, value_name = "NAME", default_value = "PATCH")]
        staging_database: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum SourceAction {
    #[command(subcommand)]
    /// Create a new source
    Create(CreateSource),

    /// List sources available to this account
    List,
}

pub async fn create(cs: &CreateSource) -> Result<()> {
    // create body for POST /sources
    // submit req
    let input = match cs {
        CreateSource::BigQuery {
            name,
            project_id,
            staging_project_id,
            credentials_key,
        } => {
            let credentials_key = std::fs::read_to_string(credentials_key)?;
            CreateSourceInput {
                name,
                source_parameters: CreateSourceParameters::BigQuery {
                    project_id,
                    staging_project_id,
                    credentials_key_b64: b64.encode(credentials_key),
                },
            }
        }
        CreateSource::Snowflake {
            name,
            organization,
            account,
            database,
            user,
            password,
            staging_database,
        } => CreateSourceInput {
            name,
            source_parameters: CreateSourceParameters::Snowflake {
                organization: organization.to_owned(),
                account,
                database,
                user,
                authentication_method: SnowflakeAuthenticationMethod::Password { password },
                staging_database,
            },
        },
    };

    let token = session::get_token()?;
    let client = Client::new(&token)?;
    client.create_source(&input).await?;

    eprintln!("Source created");
    Ok(())
}

pub async fn list() -> Result<()> {
    let token = session::get_token()?;
    let client = Client::new(&token)?;
    let sources = client.list_sources().await?.sources;

    println!("{}", serde_json::to_string_pretty(&sources)?);
    Ok(())
}
