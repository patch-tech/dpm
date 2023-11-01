use std::{fs::write, path::Path};

use anyhow::{bail, Context, Result};
use clap::Subcommand;
use inquire::{list_option::ListOption, InquireError};
use uuid7::uuid7;

use crate::{
    api,
    command::snowflake,
    descriptor::{DataPackage, DataResource, Name, TableSchema},
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

    let found_tables = match source.source_parameters {
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

    if found_tables.is_empty() {
        let mut message =
            "No tables found in the source. Creating a package with 0 tables is unsupported."
                .into();
        if refinement.is_some() {
            message = format!("{message} (tip: Remove some filter tables to widen the search.)");
        }
        bail!("{message}")
    }

    let selected_tables = select_tables_and_keys(found_tables)?;

    let descriptor = DataPackage {
        id: uuid7(),
        name: package_name.clone(),
        description: None,
        version: "0.1.0".parse().unwrap(),
        accelerated: false,
        dataset: selected_tables,
    };

    match write(output, serde_json::to_string_pretty(&descriptor).unwrap()) {
        Ok(()) => eprintln!("wrote descriptor: {}", output.display()),
        Err(e) => eprintln!("error while writing descriptor: {}", e),
    }

    Ok(())
}

fn select_tables_and_keys(
    mut tables: Vec<DataResource>,
) -> Result<Vec<DataResource>, InquireError> {
    tables.sort_unstable_by_key(|t| t.qualified_name());
    let mut selected_tables: Vec<DataResource> = Vec::new();

    // prompt user to select tables, and for each table select the PKs
    loop {
        let mut selected_table = match inquire::Select::new(
            "Select a table to add to dataset:",
            tables.iter().map(|t| t.qualified_name()).collect(),
        )
        .with_help_message(
            "↑↓ to move, enter to select, type to filter, esc to finish, ctrl+c to cancel",
        )
        .raw_prompt()
        {
            Ok(ListOption { index, .. }) => tables.remove(index),
            Err(InquireError::OperationCanceled) => break,
            Err(e) => return Err(e),
        };

        // The selected table was tenatively removed from `tables` above, but
        // will only stay removed if the user specifies a primary key for that
        // table.
        if let Some(TableSchema::Object {
            fields,
            primary_key,
            ..
        }) = selected_table.schema.as_mut()
        {
            match inquire::MultiSelect::new(
                "Select the fields that make up the table's primary key",
                fields.iter().map(|f| f.field_name().to_owned()).collect(),
            )
            .with_help_message(
                "↑↓ to move, enter to select, type to filter, esc to go back to table list, ctrl+c to cancel",
            )
            // Ensure the user selects at least one field
            .with_validator(inquire::validator::MinLengthValidator::new(1))
            .prompt()
            {
                Ok(v) => {
                    *primary_key =
                        Some(crate::descriptor::TableSchemaObjectPrimaryKey::Variant0(v));
                    selected_tables.push(selected_table);
                }
                Err(InquireError::OperationCanceled) => {
                    // User decided they don't want this table after all.
                    // Restore it to the table list.
                    tables.push(selected_table);
                    tables.sort_unstable_by_key(|t| t.qualified_name());
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }

        if tables.is_empty() {
            break;
        }
    }

    Ok(selected_tables)
}
