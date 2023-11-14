use std::{collections::HashSet, fs::write, path::Path};

use anyhow::{bail, Context, Result};
use clap::Subcommand;
use inquire::{list_option::ListOption, InquireError};
use uuid7::uuid7;

use crate::{
    api,
    descriptor::{DataPackage, DataResource, Name, TableSchema},
    env, session,
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

    if let Some(refinement) = refinement {
        // The arms of this match are intentionally no-ops. The purpose of the
        // match is to early-return if there's inconsistency between the
        // refinement used and the type of the source named in the command.
        match (refinement, &source.source_parameters) {
            (DescribeRefinement::Snowflake { .. }, api::GetSourceParameters::Snowflake { .. }) => {}
            _ => bail!(
                // TODO(PAT-4748): Update this error message
                "Incorrect `init` refinement used, given source of type {} (tip: Try `dpm init \"{}\" {} ...` instead.)",
                source.type_name(),
                source.name,
                source.type_name()
            ),
        }
    }

    let response = client.get_source_metadata(source.id).await?;

    if response.metadata.is_empty() {
        let message =
            "No tables found in the source. Creating a package with 0 tables is unsupported."
                .to_string();
        bail!("{message}")
    }

    let all_tables: Vec<DataResource> = response
        .metadata
        .into_iter()
        .filter_map(|m| {
            let table_name = m.source.path.qualified_name();
            match m.try_into() {
                Ok(table) => Some(table),
                Err(e) => {
                    eprintln!("warning: omitting table \"{}\": {}", table_name, e);
                    None
                }
            }
        })
        .collect();

    if all_tables.is_empty() {
        let message =
            "No tables usable in the source. Creating a package with 0 tables is unsupported."
                .to_string();
        bail!("{message}")
    }

    let allowed_table_indexes: HashSet<usize> = all_tables
        .iter()
        .enumerate()
        .filter_map(|(i, table)| match refinement {
            None => Some(i),
            Some(refinement) if table_satisfies_refinement(table, refinement) => Some(i),
            Some(_) => None,
        })
        .collect();

    let tables_for_prompt: Vec<DataResource> = if allowed_table_indexes.is_empty() {
        eprintln!(
            "warning: Ignoring the supplied refinement, since no tables in the source match them."
        );
        all_tables
    } else {
        filter_by_indexes(all_tables, allowed_table_indexes).collect()
    };

    let selected_tables = select_tables_and_keys(tables_for_prompt)?;

    let descriptor = DataPackage {
        id: uuid7(),
        name: package_name.to_owned(),
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

fn table_satisfies_refinement(
    candidate_table: &DataResource,
    refinement: &DescribeRefinement,
) -> bool {
    match refinement {
        DescribeRefinement::Snowflake {
            table: tables,
            schema: schemas,
        } => {
            let (candidate_table_name, candidate_table_schema) = match &candidate_table.source.path
            {
                crate::descriptor::SourcePath::Snowflake { schema, table } => (table, schema),
                _ => return false,
            };

            if tables.is_empty() && schemas.is_empty() {
                return true;
            }

            tables
                .iter()
                .any(|t| t.eq_ignore_ascii_case(candidate_table_name))
                || schemas
                    .iter()
                    .any(|s| s.eq_ignore_ascii_case(candidate_table_schema))
        }
    }
}

/// Keep only those elements whose positions in `source` are present in `indexes`.
fn filter_by_indexes<T>(
    source: impl IntoIterator<Item = T>,
    indexes: HashSet<usize>,
) -> impl Iterator<Item = T> {
    source.into_iter().enumerate().filter_map(
        move |(i, el)| {
            if indexes.contains(&i) {
                Some(el)
            } else {
                None
            }
        },
    )
}

fn select_tables_and_keys(
    mut tables: Vec<DataResource>,
) -> Result<Vec<DataResource>, InquireError> {
    // inquire doesn't have a test interface:
    // https://github.com/mikaelmello/inquire/issues/71
    //
    // Instead, during tests, assume the selection is "every table, with the
    // first field making up the primary key".
    //
    // Why not `#[cfg(test)]` here? This mocking is needed during integration
    // tests, but during `cargo test` the `dpm` bin that gets built is _not_
    // compiled with `--test`, and so the naive `#[cfg(test)]` would be
    // ineffectual.
    if env::is_test() {
        for table in tables.iter_mut() {
            if let Some(TableSchema::Object {
                fields,
                primary_key,
                ..
            }) = table.schema.as_mut()
            {
                *primary_key = Some(crate::descriptor::TableSchemaObjectPrimaryKey::Variant0(
                    vec![fields[0].field_name().to_owned()],
                ));
            }
        }

        return Ok(tables);
    }

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
