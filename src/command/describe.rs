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
    /// Create a data package descriptor file for Snowflake
    ///
    /// See https://docs.snowflake.com/en/user-guide/snowsql-start for details.
    ///
    /// If no optional arguments are given, all tables in the database given by
    /// `SNOWSQL_DATABASE` are included in the descriptor.
    #[clap(verbatim_doc_comment)]
    Snowflake {
        /// Table to include in the descriptor. May be given multiple times.
        #[arg(long)]
        table: Vec<String>,

        /// Schema whose tables to include in the descriptor. May be given multiple times.
        #[arg(long)]
        schema: Vec<String>,
    },
}

pub async fn describe(
    refinement: Option<&DescribeRefinement>,
    output: &Path,
    source_name: &str,
    package_name: &Name,
) -> Result<()> {
    let session = session::get()?;
    let client = api::Client::new(&session)?;
    let source = client
        .get_source(source_name)
        .await
        .context("Failed to get source")?;

    macro_rules! incorrect_describe {
        ($x:expr) => {
            bail!(
                "Incorrect `describe` refinement used, given source of type {} (tip: Try `dpm describe \"{}\" {} ...` instead.)",
                $x.type_name(),
                $x.name,
                $x.type_name()
            )
        };
    }

    let dataset = match source.source_parameters {
        api::GetSourceParameters::Snowflake { .. } => match refinement {
            Some(DescribeRefinement::Snowflake { table, schema }) => {
                snowflake::describe(source.id, table, schema)
            }
            None => snowflake::describe(source.id, &[], &[]),
            // Remove the following when additional source types are supported.
            #[allow(unreachable_patterns)]
            _ => incorrect_describe!(&source),
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
        dataset,
    };

    match write(output, serde_json::to_string_pretty(&descriptor).unwrap()) {
        Ok(()) => eprintln!("wrote descriptor: {}", output.display()),
        Err(e) => eprintln!("error while writing descriptor: {}", e),
    }

    Ok(())
}
