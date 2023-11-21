use std::fs;
use std::{collections::HashSet, fs::write, path::Path};

use anyhow::{bail, Context, Result};
use clap::Subcommand;
use inquire::{list_option::ListOption, InquireError};
use uuid7::uuid7;

use crate::{
    api,
    descriptor::{Dataset, Name, Table, TableSchema},
    env, session,
    util::AllowListItem,
};

pub const DEFAULT_SPEC_PATH: &str = "dataset.json";

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

impl DescribeRefinement {
    pub fn into_allow_list(self) -> Vec<AllowListItem> {
        let iter = match self {
            DescribeRefinement::Snowflake { table, schema } => {
                let table_items = table.into_iter().map(|t| AllowListItem::SnowflakeTable {
                    schema: None,
                    table: t,
                });
                let schema_items = schema.into_iter().map(AllowListItem::SnowflakeSchema);

                table_items.chain(schema_items)
            }
        };

        iter.collect()
    }
}

pub async fn init(
    source_name: &str,
    dataset_name: &Name,
    output: &Path,
    refinement: Option<DescribeRefinement>,
) -> Result<()> {
    let token = session::get_token()?;
    let client = api::Client::new(&token)?;
    let source = client
        .get_source(source_name)
        .await
        .context("Failed to get source")?;

    if let Some(refinement) = &refinement {
        // The arms of this match are intentionally no-ops. The purpose of the
        // match is to early-return if there's inconsistency between the
        // refinement used and the type of the source named in the command.
        match (refinement, &source.source_parameters) {
            (DescribeRefinement::Snowflake { .. }, api::GetSourceParameters::Snowflake { .. }) => {}
            _ => bail!(
                "Incorrect `init` refinement used, given source of type {} (tip: Try `dpm init --name \"{}\" --source \"{}\" {} ...` instead.)",
                source.type_name(),
                dataset_name,
                source.name,
                source.type_name()
            ),
        }
    }

    let response = client.get_source_metadata(source.uuid).await?;

    let allow_list = refinement.map(|r| r.into_allow_list());
    let tables_for_prompt = tables_from_metadata(response, allow_list.as_ref())?;

    let selected_tables = select_tables_and_keys(tables_for_prompt)?;

    let descriptor = Dataset {
        id: uuid7(),
        name: dataset_name.to_owned(),
        description: None,
        version: "0.1.0".parse().unwrap(),
        accelerated: false,
        tables: selected_tables,
    };

    match write(output, serde_json::to_string_pretty(&descriptor).unwrap()) {
        Ok(()) => eprintln!("wrote descriptor: {}", output.display()),
        Err(e) => eprintln!("error while writing descriptor: {}", e),
    }

    log_post_init(output, &descriptor.reference());

    Ok(())
}

fn log_post_init(descriptor_path: &Path, dataset_ref: &str) {
    let path = descriptor_path.display();
    let used_default_path = match (
        fs::canonicalize(descriptor_path),
        fs::canonicalize(DEFAULT_SPEC_PATH),
    ) {
        (Ok(actual), Ok(default)) => actual == default,
        _ => false,
    };

    let build_draft_package_command = if used_default_path {
        "build-package".into()
    } else {
        format!("build-package -s \"{}\"", path)
    };

    let publish_dataset_command = if used_default_path {
        "publish".into()
    } else {
        format!("publish -s \"{}\"", path)
    };

    let build_release_package_command = format!("build-package -d \"{}\"", dataset_ref);

    eprintln!("
Next, build a draft data package to validate your dataset:

  $ dpm {} csharp
  $ dpm {} nodejs
  $ dpm {} python

Or jump straight to publishing it:

  $ dpm {}

Publishing is the first step to making the dataset accessible to others. Once
published, authorized users may query the dataset via GraphQL, or build data
packages to query it:

  $ dpm {} csharp
  $ dpm {} nodejs
  $ dpm {} python

API docs for the built data packages may be found here: https://docs.dpm.sh/querying-data/data-packages/",
        build_draft_package_command, build_draft_package_command, build_draft_package_command,
        publish_dataset_command,
        build_release_package_command, build_release_package_command, build_release_package_command
    );
}

/// Returns a list of tables that may be used to define a dataset.
///
/// NB: This function makes no effort to check or set primary keys on tables.
///
/// If an allow list is given, it will be used to filter the set of output
/// tables. For ergonomics reasons, if no input tables are allowed by a given
/// list, a warning is logged and the function continues as though no filter had
/// been supplied.
///
/// Returns `Err` if no semantically valid set of tables can be created with the
/// given inputs.
pub fn tables_from_metadata<'a>(
    response: api::GetSourceMetadataResponse,
    allow_list: Option<impl IntoIterator<Item = &'a AllowListItem>>,
) -> Result<Vec<Table>> {
    if response.metadata.is_empty() {
        let message =
            "No tables found in the source. Creating a dataset with 0 tables is unsupported."
                .to_string();
        bail!("{message}")
    }

    let all_tables: Vec<Table> = response
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
            "No tables usable in the source. Creating a dataset with 0 tables is unsupported."
                .to_string();
        bail!("{message}")
    }

    let allowed_table_indexes: HashSet<usize> = match allow_list {
        None => (0..all_tables.len()).collect(),
        Some(list) => {
            let allow_list: Vec<&AllowListItem> = list.into_iter().collect();

            all_tables
                .iter()
                .enumerate()
                .filter_map(|(i, table)| {
                    if allow_list
                        .iter()
                        .any(|item| item.allows(&table.source.path))
                    {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect()
        }
    };

    let result: Vec<Table> = if allowed_table_indexes.is_empty() {
        eprintln!(
            "warning: Ignoring the supplied refinement, since no tables in the source match it."
        );
        all_tables
    } else {
        filter_by_indexes(all_tables, allowed_table_indexes).collect()
    };

    Ok(result)
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

fn select_tables_and_keys(mut tables: Vec<Table>) -> Result<Vec<Table>, InquireError> {
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
    let mut selected_tables: Vec<Table> = Vec::new();

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
