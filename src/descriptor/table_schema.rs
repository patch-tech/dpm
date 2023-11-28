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
    #[serde(rename = "enum", default, skip_serializing_if = "Option::is_none")]
    pub enum_: Option<Vec<String>>,
    #[doc = "An integer that specifies the maximum length of a value."]
    #[serde(rename = "maxLength", default, skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i64>,
    #[doc = "An integer that specifies the minimum length of a value."]
    #[serde(rename = "minLength", default, skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i64>,
    #[doc = "A regular expression pattern to test each value of the property against, where a truthy response indicates validity."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[doc = "Indicates whether a property must have a value for each instance."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[doc = "When `true`, each value for the property `MUST` be unique."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique: Option<bool>,
}
impl From<&Constraints> for Constraints {
    fn from(value: &Constraints) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConstraintsEnum {
    Variant0(Vec<String>),
    Variant1(Vec<f64>),
}
impl From<&ConstraintsEnum> for ConstraintsEnum {
    fn from(value: &ConstraintsEnum) -> Self {
        value.clone()
    }
}
impl From<Vec<String>> for ConstraintsEnum {
    fn from(value: Vec<String>) -> Self {
        Self::Variant0(value)
    }
}
impl From<Vec<f64>> for ConstraintsEnum {
    fn from(value: Vec<f64>) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConstraintsMaximum {
    Variant0(String),
    Variant1(f64),
}
impl From<&ConstraintsMaximum> for ConstraintsMaximum {
    fn from(value: &ConstraintsMaximum) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ConstraintsMaximum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for ConstraintsMaximum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ConstraintsMaximum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ConstraintsMaximum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for ConstraintsMaximum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<f64> for ConstraintsMaximum {
    fn from(value: f64) -> Self {
        Self::Variant1(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ConstraintsMinimum {
    Variant0(String),
    Variant1(f64),
}
impl From<&ConstraintsMinimum> for ConstraintsMinimum {
    fn from(value: &ConstraintsMinimum) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for ConstraintsMinimum {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for ConstraintsMinimum {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ConstraintsMinimum {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ConstraintsMinimum {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for ConstraintsMinimum {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<f64> for ConstraintsMinimum {
    fn from(value: f64) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "A Table Schema for this resource, compliant with the [Table Schema](/tableschema/) specification."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableSchema {
    Object {
        #[doc = "An `array` of Table Schema Field objects."]
        fields: Vec<TableSchemaField>,
        #[doc = "Values that when encountered in the source, should be considered as `null`, 'not present', or 'blank' values."]
        #[serde(
            rename = "missingValues",
            default = "defaults::table_schema_object_missing_values"
        )]
        missing_values: Vec<String>,
        #[doc = "A primary key is a field name or an array of field names, whose values `MUST` uniquely identify each row in the table."]
        #[serde(
            rename = "primaryKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        primary_key: Option<TableSchemaObjectPrimaryKey>,
    },
    String(String),
}
impl TableSchema {
    pub fn primary_key(&self) -> Option<&TableSchemaObjectPrimaryKey> {
        match self {
            TableSchema::Object { primary_key, .. } => primary_key.as_ref(),
            TableSchema::String(_) => None,
        }
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
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn table_schema_field_boolean_field_false_values() -> Vec<String> {
        vec![
            "false".to_string(),
            "False".to_string(),
            "FALSE".to_string(),
            "0".to_string(),
        ]
    }
    pub(super) fn table_schema_field_boolean_field_true_values() -> Vec<String> {
        vec![
            "true".to_string(),
            "True".to_string(),
            "TRUE".to_string(),
            "1".to_string(),
        ]
    }
    pub(super) fn table_schema_field_date_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn table_schema_field_date_time_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn table_schema_field_time_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn table_schema_object_missing_values() -> Vec<String> {
        vec!["".to_string()]
    }
}
