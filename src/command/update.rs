use std::{ffi::OsString, path::PathBuf};

use anyhow::{Context, Result};
use dialoguer::Confirm;

use crate::{
    api,
    descriptor::{Dataset, Table, TableSchemaField, TableSchemaObjectPrimaryKey},
    session,
};

use super::init;

pub async fn update(base_path: &PathBuf) -> Result<()> {
    let current_dp = Dataset::read(base_path)
        .with_context(|| format!("failed to read {}", base_path.display()))?;

    let token = session::get_token()?;
    let client = api::Client::new(&token)?;

    let source_id = current_dp
        .tables
        .iter()
        .map(|t| t.source.id)
        .next()
        .unwrap();

    let source = client
        .get_source(&source_id.to_string())
        .await
        .context("Failed to get source")?;

    let current_metadata = client.get_source_metadata(source.uuid).await?;

    // `updated_tables` is a subset of those tables currently in the dataset. It
    // may contain all the same tables, or fewer, if some have been deleted in
    // the source.
    let allow_list = current_dp.allow_list();
    let mut updated_tables = init::tables_from_metadata(current_metadata, Some(&allow_list))?;

    // Note: We don't yet support adding tables to a dataset. We just retain as
    // many of the old tables as we can, and retain their current primary keys.

    // For each of the updated tables, update its `None` primary key to be the
    // primary key of its mate in `current_dp`.
    for new_t in &mut updated_tables {
        let matching_old_t = current_dp
            .tables
            .iter()
            .find_map(|t| {
                if t.source == new_t.source {
                    Some(t.schema.as_ref().unwrap().primary_key())
                } else {
                    None
                }
            })
            // SAFETY: Because `updated_tables` is a subset of the tables currently in the dataset,
            // every new table is guaranteed to have an existing mate.
            .unwrap();

        let primary_key = &mut new_t.schema.as_mut().unwrap().primary_key;
        *primary_key = matching_old_t.cloned();
    }

    let comparisons = diff(current_dp.tables.as_slice(), updated_tables.as_slice());
    print_comparisons(&comparisons);

    if comparisons.iter().all(|c| {
        let DatasetComparison::ExistingTable { diff, .. } = c else {
            return false;
        };
        matches!(diff, TableComparison::Unchanged)
    }) {
        eprintln!("no updates to be made");
        return Ok(());
    }

    if !Confirm::new()
        .with_prompt(format!("write {}?", base_path.display()))
        .interact()?
    {
        eprintln!("update cancelled");
        return Ok(());
    }

    let mut backup_path = base_path.to_owned().into_os_string();
    backup_path.push(".backup");

    let updated_dp = Dataset {
        id: current_dp.id,
        name: current_dp.name.clone(),
        description: current_dp.description.clone(),
        version: current_dp.version.clone(),
        accelerated: current_dp.accelerated,
        tables: updated_tables,
    };

    std::fs::write(
        &backup_path,
        serde_json::to_string_pretty(&current_dp).context("serializing descriptor")?,
    )
    .context("writing backup of current descriptor")?;
    eprintln!(
        "wrote backup of previous descriptor to: {}",
        <OsString as Into<PathBuf>>::into(backup_path).display()
    );
    std::fs::write(
        base_path,
        serde_json::to_string_pretty(&updated_dp).context("serializing descriptor")?,
    )
    .context("writing updated descriptor")?;
    eprintln!("wrote updated descriptor to: {}", base_path.display());
    Ok(())
}

fn print_comparisons(comparisons: &Vec<DatasetComparison>) {
    for c in comparisons {
        match c {
            DatasetComparison::ExistingTable { table, diff } => {
                let old_name = &table.name;
                match diff {
                    TableComparison::Unchanged => (/* print nothing */),
                    TableComparison::Renamed { new_name } => {
                        eprintln!("table renamed: \"{old_name}\" => \"{new_name}\"")
                    }
                    TableComparison::Removed => eprintln!("table removed: \"{old_name}\"",),
                    TableComparison::Modified {
                        primary_key_diff,
                        field_diffs,
                    } => {
                        eprintln!("table modified: \"{old_name}\" ->");
                        if let Some(pk_diff) = primary_key_diff {
                            eprintln!(
                                "  primary key modified: {} -> {}",
                                &pk_diff.old.map_or("none".to_string(), |pk| pk.to_string()),
                                &pk_diff.new.map_or("none".to_string(), |pk| pk.to_string())
                            );
                        }
                        for diff in field_diffs {
                            if !matches!(diff, FieldComparison::Unchanged { .. }) {
                                eprint!("  ");
                            }
                            match diff {
                                FieldComparison::Added { new } => {
                                    eprintln!("field added: \"{}\"", new.name)
                                }
                                FieldComparison::Modified { old, .. } => {
                                    eprintln!("field modified: \"{}\"", old.name)
                                }
                                FieldComparison::Removed { old } => {
                                    eprintln!("field removed: \"{}\"", old.name)
                                }
                                FieldComparison::Unchanged { .. } => (/* print nothing */),
                            }
                        }
                    }
                }
            }
            DatasetComparison::NewTable { table } => {
                eprintln!("table added: \"{}\"", &table.name)
            }
        }
    }
}

/// Compares two `Dataset` instances.
fn diff<'a>(old: &'a [Table], new: &'a [Table]) -> Vec<DatasetComparison<'a>> {
    let mut old_tables: Vec<&Table> = old.iter().collect();
    let mut new_tables: Vec<&Table> = new.iter().collect();
    let mut comparisons: Vec<DatasetComparison> = vec![];

    old_tables.retain(|old_t| {
        let Some(idx) = new_tables.iter().position(|new_t| new_t == old_t) else {
            return true;
        };

        // Completely identical => Unchanged.
        comparisons.push(DatasetComparison::ExistingTable {
            table: old_t,
            diff: TableComparison::Unchanged,
        });
        new_tables.remove(idx);
        false
    });

    old_tables.retain(|old_t| {
        let Some((idx, matching_t)) = new_tables
            .iter()
            .enumerate()
            .find(|(_, new_t)| new_t.schema == old_t.schema)
        else {
            return true;
        };

        // Same schema, different something else => Renamed.
        comparisons.push(DatasetComparison::ExistingTable {
            table: old_t,
            diff: TableComparison::Renamed {
                new_name: &matching_t.name,
            },
        });
        new_tables.remove(idx);
        false
    });

    old_tables.retain(|old_t| {
        let Some((idx, new_t)) = new_tables
            .iter()
            .enumerate()
            .find(|(_, t)| t.name == old_t.name)
        else {
            return true;
        };

        // Same name, different schema (at least) => Modified.
        comparisons.push(DatasetComparison::ExistingTable {
            table: old_t,
            diff: TableComparison::Modified {
                primary_key_diff: {
                    let old_pk = old_t.schema.as_ref().unwrap().primary_key();
                    let new_pk = new_t.schema.as_ref().unwrap().primary_key();
                    if old_pk == new_pk {
                        None
                    } else {
                        Some(PrimaryKeyComparison {
                            old: old_pk,
                            new: new_pk,
                        })
                    }
                },
                field_diffs: diff_fields(old_t, new_t),
            },
        });
        new_tables.remove(idx);
        false
    });

    old_tables.retain(|old_t| {
        // No table with this name exists => Table was removed.
        comparisons.push(DatasetComparison::ExistingTable {
            table: old_t,
            diff: TableComparison::Removed,
        });
        false
    });

    while let Some(new_t) = new_tables.pop() {
        comparisons.push(DatasetComparison::NewTable { table: new_t });
    }

    comparisons
}

fn diff_fields<'a>(old_table: &'a Table, new_table: &'a Table) -> Vec<FieldComparison<'a>> {
    let old_fields = &old_table.schema.as_ref().unwrap().fields;
    let mut new_fields: Vec<&TableSchemaField> = new_table
        .schema
        .as_ref()
        .unwrap()
        .fields
        .as_slice()
        .iter()
        .collect();
    let mut comparisons: Vec<FieldComparison<'a>> = vec![];

    for old_f in old_fields {
        if let Some((idx, &new_f)) = new_fields
            .iter()
            .enumerate()
            .find(|(_, f)| f.name == old_f.name)
        {
            // Name is the same => either Unchanged or Modified.
            comparisons.push(if new_f == old_f {
                FieldComparison::Unchanged { old: old_f }
            } else {
                FieldComparison::Modified {
                    old: old_f,
                    new: new_f,
                }
            });
            new_fields.remove(idx);
        } else {
            // No field exists with the same name => Removed.
            comparisons.push(FieldComparison::Removed { old: old_f });
        }
    }

    while let Some(new_f) = new_fields.pop() {
        comparisons.push(FieldComparison::Added { new: new_f });
    }

    comparisons
}

/// The result of comparing one table to another.
enum TableComparison<'a> {
    /// Table is unchanged.
    Unchanged,
    /// Table was renamed. A table is considered renamed if it has exactly the
    /// schema it did previously. This definition is akin to git-log's
    /// `--find-renames` option with a similarity index of 100%. See
    /// https://git-scm.com/docs/git-log#Documentation/git-log.txt--Mltngt.
    Renamed { new_name: &'a String },
    /// Table was removed from the new dataset.
    Removed,
    /// Table still exists, but its columns were modified.
    Modified {
        field_diffs: Vec<FieldComparison<'a>>,
        primary_key_diff: Option<PrimaryKeyComparison<'a>>,
    },
}

struct PrimaryKeyComparison<'a> {
    pub old: Option<&'a TableSchemaObjectPrimaryKey>,
    pub new: Option<&'a TableSchemaObjectPrimaryKey>,
}

/// A comparison between the fields of two tables.
enum FieldComparison<'a> {
    /// A field with a fresh name is considered added.
    Added { new: &'a TableSchemaField },
    /// A field with an existing name but different definition.
    Modified {
        old: &'a TableSchemaField,
        #[allow(dead_code)]
        new: &'a TableSchemaField,
    },
    /// A field with an existing name that no longer exists.
    Removed { old: &'a TableSchemaField },
    /// Field is unchanged.
    #[allow(dead_code)]
    Unchanged { old: &'a TableSchemaField },
}

/// The difference between two datasets can be described as a set of
/// `DatasetComparison` structs.
enum DatasetComparison<'a> {
    /// A description of possible change undergone by a table that appeared in
    /// the old dataset.
    ExistingTable {
        /// Table as it existed in the old dataset.
        table: &'a Table,
        /// A description of how this table is different in the new dataset.
        diff: TableComparison<'a>,
    },
    /// A description of a table that exists only in the new dataset.
    NewTable { table: &'a Table },
}
