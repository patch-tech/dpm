use std::{fs::write, path::Path};

use anyhow::{bail, Context, Result};
use clap::Subcommand;
use uuid7::uuid7;

use crate::{
    api,
    command::snowflake,
    descriptor::{DataPackage, Name},
    session,
};

#[derive(Subcommand, Debug)]
pub enum DescribeRefinement {
    /// Filter the tables to include.
    ///
    /// A table is included in the output descriptor only if it matches any of
    /// the arguments given here. If none of these arguments are given, all
    /// tables in the database named by the source will be included.
    Snowflake {
        /// Table to include in the descriptor. May be given multiple times.
        #[arg(long)]
        table: Vec<String>,

        /// Schema whose tables to include in the descriptor. May be given multiple times.
        #[arg(long)]
        schema: Vec<String>,
    },
}

pub async fn init(
    source_name: &str,
    package_name: &Name,
    output: &Path,
    refinement: Option<&DescribeRefinement>,
) -> Result<()> {
    let token = session::get_token()?;
    let client = api::Client::new(&token)?;
    let source = client
        .get_source(source_name)
        .await
        .context("Failed to get source")?;

    macro_rules! incorrect_refinement {
        ($x:expr) => {
            bail!(
                "Incorrect `init` refinement used, given source of type {} (tip: Try `dpm init \"{}\" {} ...` instead.)",
                $x.type_name(),
                $x.name,
                $x.type_name()
            )
        };
    }

    let dataset = match source.source_parameters {
        #[allow(unused_variables)] // TODO(PAT-4696): Remove this allowance
        api::GetSourceParameters::BigQuery {
            project_id,
            staging_project_id,
        } => bail!("init with BigQuery not yet supported"), // TODO(PAT-4696)
        api::GetSourceParameters::Snowflake { .. } => match refinement {
            Some(DescribeRefinement::Snowflake { table, schema }) => {
                snowflake::describe(source.id, table, schema)
            }
            None => snowflake::describe(source.id, &[], &[]),
            // Remove the following when additional source types are supported.
            #[allow(unreachable_patterns)]
            _ => incorrect_refinement!(&source),
        },
    }
    .await?;

    if dataset.is_empty() {
        let mut message =
            "No tables found in the source. Creating a package with 0 tables is unsupported."
                .into();
        if refinement.is_some() {
            message = format!("{message} (tip: Remove some filter tables to widen the search.)");
        }
        panic!("{message}")
    }

    let descriptor = DataPackage {
        id: uuid7(),
        name: package_name.clone(),
        description: None,
        version: "0.1.0".parse().unwrap(),
        accelerated: false,
        dataset,
    };

    match write(output, serde_json::to_string_pretty(&descriptor).unwrap()) {
        Ok(()) => eprintln!("wrote descriptor: {}", output.display()),
        Err(e) => eprintln!("error while writing descriptor: {}", e),
    }

    Ok(())
}
