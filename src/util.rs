use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourcePath {
    #[serde(rename = "bigquery")]
    BigQuery {
        table: String,
    },
    Snowflake {
        schema: String,
        table: String,
    },
}

impl SourcePath {
    /// Returns a string that unambiguously identifies a table within a source.
    pub fn qualified_name(&self) -> String {
        match &self {
            SourcePath::BigQuery { table } => table.to_owned(),
            SourcePath::Snowflake { schema, table } => format!("{}.{}", schema, table),
        }
    }
}

/// A predicate over `SourcePath`.`
pub enum AllowListItem {
    /// Allows any BigQuery table with the given name
    BigQueryTable(String),
    /// Allows any table in the given Snowflake schema
    SnowflakeSchema(String),
    /// Allows any table with a given name, optionally constrained further to
    /// belong in a given schema.
    SnowflakeTable {
        schema: Option<String>,
        table: String,
    },
}

impl AllowListItem {
    pub fn allows(&self, candidate: &SourcePath) -> bool {
        match (self, candidate) {
            (AllowListItem::BigQueryTable(target), SourcePath::BigQuery { table }) => {
                table == target
            }

            (AllowListItem::SnowflakeSchema(target), SourcePath::Snowflake { schema, .. }) => {
                schema == target
            }

            (
                AllowListItem::SnowflakeTable {
                    schema: target_schema,
                    table: target_table,
                },
                SourcePath::Snowflake { schema, table },
            ) => {
                target_schema
                    .as_ref()
                    .map_or(true, |target_schema| schema == target_schema)
                    && table == target_table
            }

            _ => false,
        }
    }
}
