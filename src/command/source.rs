use anyhow::Result;
use clap::Subcommand;

use crate::{
    api::{Client, CreateSourceInput, CreateSourceParameters, SnowflakeAuthenticationMethod},
    session,
};

use super::snowflake;

#[derive(Debug, Subcommand)]
pub enum CreateSource {
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

        /// Warehouse in which queries will be run.
        #[arg[long, value_name = "NAME"]]
        warehouse: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum SourceAction {
    #[command(subcommand)]
    Create(CreateSource),

    List,
}

pub async fn create(cs: &CreateSource) -> Result<()> {
    // create body for POST /sources
    // submit req
    let input = match cs {
        CreateSource::Snowflake {
            name,
            organization,
            account,
            database,
            user,
            password,
            warehouse,
        } => CreateSourceInput {
            name,
            source_parameters: CreateSourceParameters::Snowflake {
                organization: organization.to_owned(),
                account,
                database,
                user,
                authentication_method: SnowflakeAuthenticationMethod::Password { password },
                warehouse,
            },
        },
    };

    let session = session::get().await?;
    let client = Client::new(&session)?;
    client.create_source(&input).await?;

    eprintln!("Source created");
    Ok(())
}

pub async fn list() -> Result<()> {
    // GET /sources, get back a Vec<something>
    // print their names
    let session = session::get().await?;
    let client = Client::new(&session)?;
    let sources = client.list_sources().await?.sources;

    println!("{}", serde_json::to_string_pretty(&sources)?);
    Ok(())
}
