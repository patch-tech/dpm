use anyhow::bail;
use semver::Version;
use serde::{Deserialize, Serialize};
use uuid7::Uuid;

use super::table_schema::TableSchema;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum TableLocation {
    Patch,
    Snowflake {
        organization_name: String,
        account_name: String,
        database: String,
        schema: String,
        table: String,
    },
}

#[derive(Deserialize, Serialize)]
pub struct DataPackage {
    pub id: Uuid,
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
    pub location: TableLocation,
    pub path: Option<String>,
    pub schema: Option<TableSchema>,
}
