//! Generated with https://github.com/oxidecomputer/typify/tree/v0.0.13 from the
//! JSON Schema at https://specs.frictionlessdata.io/table-schema/, accessed on
//! 2023-06-04. On that day the page at that URL said "Updated 5 October 2021".

#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::derivable_impls)]
#![allow(clippy::enum_variant_names)]
#![allow(dead_code)]

use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[doc = "The following constraints are supported for `string` fields."]
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Constraints {
    #[doc = "Indicates whether a property must have a value for each instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[doc = "A Table Schema for this resource, compliant with the [Table Schema](/tableschema/) specification."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct TableSchema {
    #[doc = "An `array` of Table Schema Field objects."]
    pub fields: Vec<TableSchemaField>,
    #[doc = "A primary key is a field name or an array of field names, whose values `MUST` uniquely identify each row in the table."]
    #[serde(
        rename = "primaryKey",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_key: Option<TableSchemaObjectPrimaryKey>,
}
impl TableSchema {
    pub fn primary_key(&self) -> Option<&TableSchemaObjectPrimaryKey> {
        self.primary_key.as_ref()
    }
}
impl From<&TableSchema> for TableSchema {
    fn from(value: &TableSchema) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    String,
    Number,
    Date,
    Time,
    DateTime,
    Boolean,
    Array,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub struct TableSchemaField {
    #[serde(rename = "type")]
    pub type_: FieldType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "A name for this field."]
    pub name: String,
}

impl From<&TableSchemaField> for TableSchemaField {
    fn from(value: &TableSchemaField) -> Self {
        value.clone()
    }
}
#[doc = "A primary key is a field name or an array of field names, whose values `MUST` uniquely identify each row in the table."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableSchemaObjectPrimaryKey {
    Variant0(Vec<String>),
    Variant1(String),
}
impl From<&TableSchemaObjectPrimaryKey> for TableSchemaObjectPrimaryKey {
    fn from(value: &TableSchemaObjectPrimaryKey) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for TableSchemaObjectPrimaryKey {
    fn from(value: Vec<String>) -> Self {
        Self::Variant0(value)
    }
}
impl Display for TableSchemaObjectPrimaryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableSchemaObjectPrimaryKey::Variant0(fields) => f.write_str(&fields.join(", ")),
            TableSchemaObjectPrimaryKey::Variant1(field) => {
                f.write_fmt(format_args!("({})", field))
            }
        }
    }
}
