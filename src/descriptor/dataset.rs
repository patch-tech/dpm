use std::{fmt::Display, fs::File, io::BufReader, path::Path};

use anyhow::{bail, Context, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use uuid7::Uuid as Uuid7;

use super::{table_schema::TableSchema, Constraints, TableSchemaField};
use crate::{
    api,
    util::{AllowListItem, SourcePath},
};

/// The logical address of a table.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TableSource {
    /// Reference to a DPM Cloud source entity.
    pub id: Uuid,
    /// Information sufficient to find a table within a source.
    pub path: SourcePath,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Dataset {
    pub id: Uuid7,
    pub name: Name,
    pub description: Option<String>,
    pub version: Version,
    #[serde(default)]
    pub accelerated: bool,
    #[serde(rename = "dataset")]
    pub tables: Vec<Table>,
}

impl Dataset {
    /// Reads dataset.json at path and returns a deserialized instance of Dataset.
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let dataset = serde_json::from_reader(reader).context("deserialization failed")?;
        Ok(dataset)
    }

    /// Returns an allow list that may be used to recover the set of tables in
    /// `self` from a larger collection.
    pub fn allow_list(&self) -> Vec<AllowListItem> {
        self.tables
            .iter()
            .map(|table| match table.source.path.to_owned() {
                SourcePath::BigQuery { table } => AllowListItem::BigQueryTable(table),
                SourcePath::Snowflake { schema, table } => AllowListItem::SnowflakeTable {
                    schema: Some(schema),
                    table,
                },
            })
            .collect()
    }

    /// Returns the canonical identifier for the dataset/version described by
    /// `self`.
    pub fn reference(&self) -> String {
        format!("{}@{}", self.name, self.version)
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
pub struct Table {
    pub name: String,
    pub description: Option<String>,
    #[doc = "Where the table data resides"]
    pub source: TableSource,
    pub schema: Option<TableSchema>,
}

impl Table {
    /// Returns a string that unambiguously identifies a table within a source.
    pub fn qualified_name(&self) -> String {
        self.source.path.qualified_name()
    }
}

impl TryFrom<api::TableMetadata> for Table {
    type Error = String;

    fn try_from(value: api::TableMetadata) -> std::result::Result<Self, Self::Error> {
        Ok(Table {
            name: match &value.source.path {
                SourcePath::BigQuery { table } => table.to_owned(),
                // TODO(PAT-4860): Use schema to prevent collisions in table name
                SourcePath::Snowflake {
                    schema: _schema,
                    table,
                } => table.to_owned(),
            },
            description: None,
            source: value.source,
            schema: Some(value.schema.try_into()?),
        })
    }
}

impl TryFrom<api::TableSchema> for TableSchema {
    type Error = String;

    fn try_from(value: api::TableSchema) -> std::result::Result<Self, Self::Error> {
        let fields: Vec<TableSchemaField> = value
            .fields
            .into_iter()
            .filter_map(|f| {
                let field_name = f.name.clone();

                match f.try_into() {
                    Ok(f) => Some(f),
                    Err(e) => {
                        eprintln!("warning: omitting field \"{}\": {}", field_name, e);
                        None
                    }
                }
            })
            .collect();

        if fields.is_empty() {
            return Err("no fields are usable".into());
        }

        Ok(TableSchema::Object {
            fields,
            missing_values: Vec::new(),
            primary_key: None,
        })
    }
}

#[derive(Debug)]
pub enum FieldError {
    UnsupportedSourceType(String),
    UnrecognizedDpmType(String),
}

impl Display for FieldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldError::UnsupportedSourceType(source_type) => f.write_fmt(format_args!(
                "unsupported source data type \"{}\"; consider submitting a feature request",
                source_type
            )),
            FieldError::UnrecognizedDpmType(dpm_type) => f.write_fmt(format_args!(
                "unrecognized dpm data type \"{}\" (tip: Upgrade `dpm` and try again)",
                dpm_type
            )),
        }
    }
}

impl TryFrom<api::FieldSchema> for TableSchemaField {
    type Error = FieldError;

    fn try_from(value: api::FieldSchema) -> std::result::Result<Self, Self::Error> {
        let Some(dpm_beta_type) = value.dpm_beta_type else {
            return Err(FieldError::UnsupportedSourceType(value.source_type));
        };
        let Ok(dpm_beta_type) = dpm_beta_type.parse::<api::DpmBetaType>() else {
            return Err(FieldError::UnrecognizedDpmType(dpm_beta_type));
        };

        let base_constraints = Constraints {
            required: Some(!value.nullable),
            ..Default::default()
        };

        Ok(match dpm_beta_type {
            api::DpmBetaType::String => TableSchemaField::String {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
            api::DpmBetaType::Boolean => TableSchemaField::Boolean {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
            api::DpmBetaType::Number => TableSchemaField::Number {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
            api::DpmBetaType::Date => TableSchemaField::Date {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
            api::DpmBetaType::Time => TableSchemaField::Time {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
            api::DpmBetaType::DateTime => TableSchemaField::DateTime {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
            api::DpmBetaType::Array => TableSchemaField::Array {
                constraints: Some(base_constraints),
                description: None,
                name: value.name,
            },
        })
    }
}
