use std::{fmt::Display, fs::File, io::BufReader, path::Path};

use anyhow::{bail, Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use uuid7::Uuid as Uuid7;

use super::{
    table_schema::TableSchema, ArrayFieldType, BooleanFieldType, Constraints, DateFieldType,
    DateTimeFieldType, NumberFieldType, StringFieldFormat, StringFieldType, TableSchemaField,
    TimeFieldType,
};
use crate::api;

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

/// The logical address of a table.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TableSource {
    /// Reference to a DPM Cloud source entity.
    pub id: Uuid,
    /// Information sufficient to find a table within a source.
    pub path: SourcePath,
}
impl TableSource {
    pub fn new(id: Uuid, path: SourcePath) -> Self {
        TableSource { id, path }
    }
}

#[derive(Deserialize, Serialize)]
pub struct DataPackage {
    pub id: Uuid7,
    pub name: Name,
    pub description: Option<String>,
    pub version: Version,
    #[serde(default)]
    pub accelerated: bool,
    pub dataset: Vec<DataResource>,
}

impl DataPackage {
    /// Reads datapackage.json at path and returns a deserialized instance of DataPackage.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let data_package = serde_json::from_reader(reader).context("deserialization failed")?;
        Ok(data_package)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name(String);

impl std::ops::Deref for Name {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::str::FromStr for Name {
    type Err = anyhow::Error;
    fn from_str(value: &str) -> Result<Self, anyhow::Error> {
        if regress::Regex::new("^[A-Za-z0-9-_]+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            bail!("doesn't match pattern \"^[A-Za-z0-9-_]+$\"");
        }
        Ok(Self(value.to_string()))
    }
}

#[doc = "Data Resource."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DataResource {
    pub name: String,
    pub description: Option<String>,
    #[doc = "Where the table data resides"]
    pub source: TableSource,
    pub schema: Option<TableSchema>,
}

impl DataResource {
    /// Returns a string that unambiguously identifies a table within a source.
    pub fn qualified_name(&self) -> String {
        match &self.source.path {
            SourcePath::Snowflake { schema, table } => format!("{}.{}", schema, table),
        }
    }
}
