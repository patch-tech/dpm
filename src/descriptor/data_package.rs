use anyhow::bail;
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use uuid7::Uuid as Uuid7;

use super::table_schema::TableSchema;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SourcePath {
    Snowflake { schema: String, table: String },
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
    pub dataset: Vec<DataResource>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Name(String);

impl std::ops::Deref for Name {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
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
