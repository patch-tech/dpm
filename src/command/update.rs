use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use dialoguer::Confirm;

use crate::descriptor::{DataPackage, DataResource, TableSchema, TableSchemaField};

use super::snowflake;

/// Reads datapackage.json at path and returns a deserialized instance of DataPackage.
fn read_data_package<P: AsRef<Path>>(path: P) -> Result<DataPackage> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let data_package = serde_json::from_reader(reader).context("deserialization failed")?;
    Ok(data_package)
}

pub async fn update(base_path: &PathBuf) -> Result<()> {
    let current_dp = read_data_package(base_path)
        .with_context(|| format!("failed to read {}", base_path.to_string_lossy()))?;

    let package_name: &str = current_dp.name.as_ref().unwrap();
    let tables = current_dp
        .resources
        .iter()
        .map(|t| match &t.location {
            crate::descriptor::TableLocation::Patch => todo!(),
            crate::descriptor::TableLocation::Snowflake { table, .. } => table.clone(),
        })
        .collect();

    let updated = snowflake::describe(package_name, tables, vec![]).await;

    let comparisons = diff(&current_dp, &updated);
    print_comparisons(&comparisons);

    if comparisons.iter().all(|c| {
        let DatasetComparison::ExistingTable { diff, .. } = c
        else { return false; };
        matches!(diff, TableComparison::Unchanged)
    }) {
        eprintln!("no updates to be made");
        return Ok(());
    }

    if !Confirm::new()
        .with_prompt(format!("write {}?", base_path.to_string_lossy()))
        .interact()?
    {
        eprintln!("update cancelled");
        return Ok(());
    }

    let mut backup_path = base_path.to_owned().into_os_string();
    backup_path.push(".backup");

    std::fs::write(
        &backup_path,
        serde_json::to_string_pretty(&current_dp).context("serializing descriptor")?,
    )
    .context("writing backup of current descriptor")?;
    eprintln!(
        "wrote backup of previous descriptor to: {}",
        backup_path.to_string_lossy()
    );
    std::fs::write(
        base_path,
        serde_json::to_string_pretty(&updated).context("serializing descriptor")?,
    )
    .context("writing updated descriptor")?;
    eprintln!(
        "wrote updated descriptor to: {}",
        base_path.to_string_lossy()
    );
    Ok(())
}

fn print_comparisons(comparisons: &Vec<DatasetComparison>) {
    for c in comparisons {
        match c {
            DatasetComparison::ExistingTable { table, diff } => {
                let old_name = table.name.as_ref().unwrap();
                match diff {
                    TableComparison::Unchanged => (/* print nothing */),
                    TableComparison::Renamed { new_name } => {
                        eprintln!("table renamed: \"{old_name}\" => \"{new_name}\"")
                    }
                    TableComparison::Removed => eprintln!("table removed: \"{old_name}\"",),
                    TableComparison::Modified { field_diffs } => {
                        eprintln!("table modified: \"{old_name}\" ->");
                        for diff in field_diffs {
                            if !matches!(diff, FieldComparison::Unchanged { .. }) {
                                eprint!("  ");
                            }
                            match diff {
                                FieldComparison::Added { new } => {
                                    eprintln!("field added: \"{}\"", field_name(new))
                                }
                                FieldComparison::Modified { old, .. } => {
                                    eprintln!("field modified: \"{}\"", field_name(old))
                                }
                                FieldComparison::Removed { old } => {
                                    eprintln!("field removed: \"{}\"", field_name(old))
                                }
                                FieldComparison::Unchanged { .. } => (/* print nothing */),
                            }
                        }
                    }
                }
            }
            DatasetComparison::NewTable { table } => {
                eprintln!("table added: \"{}\"", table.name.as_ref().unwrap())
            }
        }
    }
}

/// Compares two `DataPackage` instances.
fn diff<'a>(old: &'a DataPackage, new: &'a DataPackage) -> Vec<DatasetComparison<'a>> {
    let mut old_tables: Vec<&DataResource> = old.resources.iter().collect();
    let mut new_tables: Vec<&DataResource> = new.resources.as_slice().iter().collect();
    let mut comparisons: Vec<DatasetComparison> = vec![];

    old_tables.retain(|old_t| {
        let Some(idx) = new_tables.iter().position(|new_t| new_t == old_t)
        else { return true; };

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
        else { return true; };

        // Same schema, different something else => Renamed.
        comparisons.push(DatasetComparison::ExistingTable {
            table: old_t,
            diff: TableComparison::Renamed {
                new_name: matching_t.name.as_ref().unwrap(),
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
        else { return true; };

        // Same name, different schema (at least) => Modified.
        comparisons.push(DatasetComparison::ExistingTable {
            table: old_t,
            diff: TableComparison::Modified {
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

fn diff_fields<'a>(
    old_table: &'a DataResource,
    new_table: &'a DataResource,
) -> Vec<FieldComparison<'a>> {
    let old_fields = match old_table.schema.as_ref().unwrap() {
        TableSchema::Object { fields, .. } => fields,
        TableSchema::String(_) => unreachable!(),
    };
    let mut new_fields: Vec<&TableSchemaField> = match new_table.schema.as_ref().unwrap() {
        TableSchema::Object { fields, .. } => fields.as_slice().iter().collect(),
        TableSchema::String(_) => unreachable!(),
    };
    let mut comparisons: Vec<FieldComparison<'a>> = vec![];

    for old_f in old_fields {
        if let Some((idx, &new_f)) = new_fields
            .iter()
            .enumerate()
            .find(|(_, f)| field_name(f) == field_name(old_f))
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

/// Extracts the name of the field.
///
/// TODO(PAT-3446): Obviate the need for this.
/// See also:
/// - https://stackoverflow.com/questions/49186751/sharing-a-common-value-in-all-enum-values
/// - https://users.rust-lang.org/t/destructuring-a-common-field-from-many-enum-variants/60997
fn field_name(f: &TableSchemaField) -> &String {
    match f {
        TableSchemaField::StringField { name, .. } => name,
        TableSchemaField::NumberField { name, .. } => name,
        TableSchemaField::IntegerField { name, .. } => name,
        TableSchemaField::DateField { name, .. } => name,
        TableSchemaField::TimeField { name, .. } => name,
        TableSchemaField::YearField { name, .. } => name,
        TableSchemaField::YearMonthField { name, .. } => name,
        TableSchemaField::BooleanField { name, .. } => name,
        TableSchemaField::ObjectField { name, .. } => name,
        TableSchemaField::GeoPointField { name, .. } => name,
        TableSchemaField::GeoJsonField { name, .. } => name,
        TableSchemaField::ArrayField { name, .. } => name,
        TableSchemaField::DurationField { name, .. } => name,
        TableSchemaField::AnyField { name, .. } => name,
        TableSchemaField::DateTimeField { name, .. } => name,
    }
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
    },
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
        table: &'a DataResource,
        /// A description of how this table is different in the new dataset.
        diff: TableComparison<'a>,
    },
    /// A description of a table that exists only in the new dataset.
    NewTable { table: &'a DataResource },
}
