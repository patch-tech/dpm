//! Generated with https://github.com/oxidecomputer/typify/tree/v0.0.13 from the
//! JSON Schema at https://specs.frictionlessdata.io/table-schema/, accessed on
//! 2023-06-04. On that day the page at that URL said "Updated 5 October 2021".

#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[doc = "Any value is accepted, including values that are not captured by the type/format/constraint requirements of the specification."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AnyField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `any`."]
    #[serde(rename = "type")]
    pub type_: AnyFieldType,
}
impl From<&AnyField> for AnyField {
    fn from(value: &AnyField) -> Self {
        value.clone()
    }
}
impl AnyField {
    pub fn builder() -> builder::AnyField {
        builder::AnyField::default()
    }
}
#[doc = "The type keyword, which `MUST` be a value of `any`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum AnyFieldType {
    #[serde(rename = "any")]
    Any,
}
impl From<&AnyFieldType> for AnyFieldType {
    fn from(value: &AnyFieldType) -> Self {
        value.clone()
    }
}
impl ToString for AnyFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Any => "any".to_string(),
        }
    }
}
impl std::str::FromStr for AnyFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "any" => Ok(Self::Any),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for AnyFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for AnyFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for AnyFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains data which can be parsed as a valid JSON array."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArrayField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `array`: only `default` is allowed."]
    #[serde(default = "defaults::array_field_format")]
    pub format: ArrayFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `array`."]
    #[serde(rename = "type")]
    pub type_: ArrayFieldType,
}
impl From<&ArrayField> for ArrayField {
    fn from(value: &ArrayField) -> Self {
        value.clone()
    }
}
impl ArrayField {
    pub fn builder() -> builder::ArrayField {
        builder::ArrayField::default()
    }
}
#[doc = "There are no format keyword options for `array`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ArrayFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&ArrayFieldFormat> for ArrayFieldFormat {
    fn from(value: &ArrayFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for ArrayFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for ArrayFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ArrayFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ArrayFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ArrayFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for ArrayFieldFormat {
    fn default() -> Self {
        ArrayFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `array`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ArrayFieldType {
    #[serde(rename = "array")]
    Array,
}
impl From<&ArrayFieldType> for ArrayFieldType {
    fn from(value: &ArrayFieldType) -> Self {
        value.clone()
    }
}
impl ToString for ArrayFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Array => "array".to_string(),
        }
    }
}
impl std::str::FromStr for ArrayFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "array" => Ok(Self::Array),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ArrayFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ArrayFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ArrayFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains boolean (true/false) data."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BooleanField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[serde(
        rename = "falseValues",
        default = "defaults::boolean_field_false_values"
    )]
    pub false_values: Vec<String>,
    #[doc = "There are no format keyword options for `boolean`: only `default` is allowed."]
    #[serde(default = "defaults::boolean_field_format")]
    pub format: BooleanFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "trueValues", default = "defaults::boolean_field_true_values")]
    pub true_values: Vec<String>,
    #[doc = "The type keyword, which `MUST` be a value of `boolean`."]
    #[serde(rename = "type")]
    pub type_: BooleanFieldType,
}
impl From<&BooleanField> for BooleanField {
    fn from(value: &BooleanField) -> Self {
        value.clone()
    }
}
impl BooleanField {
    pub fn builder() -> builder::BooleanField {
        builder::BooleanField::default()
    }
}
#[doc = "There are no format keyword options for `boolean`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BooleanFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&BooleanFieldFormat> for BooleanFieldFormat {
    fn from(value: &BooleanFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for BooleanFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for BooleanFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BooleanFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BooleanFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BooleanFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for BooleanFieldFormat {
    fn default() -> Self {
        BooleanFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `boolean`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum BooleanFieldType {
    #[serde(rename = "boolean")]
    Boolean,
}
impl From<&BooleanFieldType> for BooleanFieldType {
    fn from(value: &BooleanFieldType) -> Self {
        value.clone()
    }
}
impl ToString for BooleanFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Boolean => "boolean".to_string(),
        }
    }
}
impl std::str::FromStr for BooleanFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "boolean" => Ok(Self::Boolean),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for BooleanFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for BooleanFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for BooleanFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The following constraints are supported for `string` fields."]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
impl Constraints {
    pub fn builder() -> builder::Constraints {
        builder::Constraints::default()
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
#[doc = "The field contains temporal date values."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "The format keyword options for `date` are `default`, `any`, and `{PATTERN}`."]
    #[serde(default = "defaults::date_field_format")]
    pub format: serde_json::Value,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `date`."]
    #[serde(rename = "type")]
    pub type_: DateFieldType,
}
impl From<&DateField> for DateField {
    fn from(value: &DateField) -> Self {
        value.clone()
    }
}
impl DateField {
    pub fn builder() -> builder::DateField {
        builder::DateField::default()
    }
}
#[doc = "The type keyword, which `MUST` be a value of `date`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DateFieldType {
    #[serde(rename = "date")]
    Date,
}
impl From<&DateFieldType> for DateFieldType {
    fn from(value: &DateFieldType) -> Self {
        value.clone()
    }
}
impl ToString for DateFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Date => "date".to_string(),
        }
    }
}
impl std::str::FromStr for DateFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "date" => Ok(Self::Date),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DateFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DateFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DateFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains temporal datetime values."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DateTimeField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "The format keyword options for `datetime` are `default`, `any`, and `{PATTERN}`."]
    #[serde(default = "defaults::date_time_field_format")]
    pub format: serde_json::Value,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `datetime`."]
    #[serde(rename = "type")]
    pub type_: DateTimeFieldType,
}
impl From<&DateTimeField> for DateTimeField {
    fn from(value: &DateTimeField) -> Self {
        value.clone()
    }
}
impl DateTimeField {
    pub fn builder() -> builder::DateTimeField {
        builder::DateTimeField::default()
    }
}
#[doc = "The type keyword, which `MUST` be a value of `datetime`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DateTimeFieldType {
    #[serde(rename = "datetime")]
    Datetime,
}
impl From<&DateTimeFieldType> for DateTimeFieldType {
    fn from(value: &DateTimeFieldType) -> Self {
        value.clone()
    }
}
impl ToString for DateTimeFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Datetime => "datetime".to_string(),
        }
    }
}
impl std::str::FromStr for DateTimeFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "datetime" => Ok(Self::Datetime),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DateTimeFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DateTimeFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DateTimeFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains a duration of time."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DurationField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `duration`: only `default` is allowed."]
    #[serde(default = "defaults::duration_field_format")]
    pub format: DurationFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `duration`."]
    #[serde(rename = "type")]
    pub type_: DurationFieldType,
}
impl From<&DurationField> for DurationField {
    fn from(value: &DurationField) -> Self {
        value.clone()
    }
}
impl DurationField {
    pub fn builder() -> builder::DurationField {
        builder::DurationField::default()
    }
}
#[doc = "There are no format keyword options for `duration`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DurationFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&DurationFieldFormat> for DurationFieldFormat {
    fn from(value: &DurationFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for DurationFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for DurationFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DurationFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DurationFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DurationFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for DurationFieldFormat {
    fn default() -> Self {
        DurationFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `duration`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum DurationFieldType {
    #[serde(rename = "duration")]
    Duration,
}
impl From<&DurationFieldType> for DurationFieldType {
    fn from(value: &DurationFieldType) -> Self {
        value.clone()
    }
}
impl ToString for DurationFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Duration => "duration".to_string(),
        }
    }
}
impl std::str::FromStr for DurationFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "duration" => Ok(Self::Duration),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for DurationFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for DurationFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for DurationFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains a JSON object according to GeoJSON or TopoJSON"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeoJsonField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "The format keyword options for `geojson` are `default` and `topojson`."]
    #[serde(default = "defaults::geo_json_field_format")]
    pub format: GeoJsonFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `geojson`."]
    #[serde(rename = "type")]
    pub type_: GeoJsonFieldType,
}
impl From<&GeoJsonField> for GeoJsonField {
    fn from(value: &GeoJsonField) -> Self {
        value.clone()
    }
}
impl GeoJsonField {
    pub fn builder() -> builder::GeoJsonField {
        builder::GeoJsonField::default()
    }
}
#[doc = "The format keyword options for `geojson` are `default` and `topojson`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GeoJsonFieldFormat {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "topojson")]
    Topojson,
}
impl From<&GeoJsonFieldFormat> for GeoJsonFieldFormat {
    fn from(value: &GeoJsonFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for GeoJsonFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Topojson => "topojson".to_string(),
        }
    }
}
impl std::str::FromStr for GeoJsonFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            "topojson" => Ok(Self::Topojson),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for GeoJsonFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GeoJsonFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GeoJsonFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for GeoJsonFieldFormat {
    fn default() -> Self {
        GeoJsonFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `geojson`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GeoJsonFieldType {
    #[serde(rename = "geojson")]
    Geojson,
}
impl From<&GeoJsonFieldType> for GeoJsonFieldType {
    fn from(value: &GeoJsonFieldType) -> Self {
        value.clone()
    }
}
impl ToString for GeoJsonFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Geojson => "geojson".to_string(),
        }
    }
}
impl std::str::FromStr for GeoJsonFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "geojson" => Ok(Self::Geojson),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for GeoJsonFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GeoJsonFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GeoJsonFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains data describing a geographic point."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GeoPointField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "The format keyword options for `geopoint` are `default`,`array`, and `object`."]
    #[serde(default = "defaults::geo_point_field_format")]
    pub format: GeoPointFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `geopoint`."]
    #[serde(rename = "type")]
    pub type_: GeoPointFieldType,
}
impl From<&GeoPointField> for GeoPointField {
    fn from(value: &GeoPointField) -> Self {
        value.clone()
    }
}
impl GeoPointField {
    pub fn builder() -> builder::GeoPointField {
        builder::GeoPointField::default()
    }
}
#[doc = "The format keyword options for `geopoint` are `default`,`array`, and `object`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GeoPointFieldFormat {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "array")]
    Array,
    #[serde(rename = "object")]
    Object,
}
impl From<&GeoPointFieldFormat> for GeoPointFieldFormat {
    fn from(value: &GeoPointFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for GeoPointFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Array => "array".to_string(),
            Self::Object => "object".to_string(),
        }
    }
}
impl std::str::FromStr for GeoPointFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            "array" => Ok(Self::Array),
            "object" => Ok(Self::Object),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for GeoPointFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GeoPointFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GeoPointFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for GeoPointFieldFormat {
    fn default() -> Self {
        GeoPointFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `geopoint`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum GeoPointFieldType {
    #[serde(rename = "geopoint")]
    Geopoint,
}
impl From<&GeoPointFieldType> for GeoPointFieldType {
    fn from(value: &GeoPointFieldType) -> Self {
        value.clone()
    }
}
impl ToString for GeoPointFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Geopoint => "geopoint".to_string(),
        }
    }
}
impl std::str::FromStr for GeoPointFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "geopoint" => Ok(Self::Geopoint),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for GeoPointFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for GeoPointFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for GeoPointFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains integers - that is whole numbers."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IntegerField {
    #[doc = "a boolean field with a default of `true`. If `true` the physical contents of this field must follow the formatting constraints already set out. If `false` the contents of this field may contain leading and/or trailing non-numeric characters (which implementors MUST therefore strip). The purpose of `bareNumber` is to allow publishers to publish numeric data that contains trailing characters such as percentages e.g. `95%` or leading characters such as currencies e.g. `€95` or `EUR 95`. Note that it is entirely up to implementors what, if anything, they do with stripped text."]
    #[serde(rename = "bareNumber", default = "defaults::default_bool::<true>")]
    pub bare_number: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `integer`: only `default` is allowed."]
    #[serde(default = "defaults::integer_field_format")]
    pub format: IntegerFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `integer`."]
    #[serde(rename = "type")]
    pub type_: IntegerFieldType,
}
impl From<&IntegerField> for IntegerField {
    fn from(value: &IntegerField) -> Self {
        value.clone()
    }
}
impl IntegerField {
    pub fn builder() -> builder::IntegerField {
        builder::IntegerField::default()
    }
}
#[doc = "There are no format keyword options for `integer`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IntegerFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&IntegerFieldFormat> for IntegerFieldFormat {
    fn from(value: &IntegerFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for IntegerFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for IntegerFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IntegerFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntegerFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntegerFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for IntegerFieldFormat {
    fn default() -> Self {
        IntegerFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `integer`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum IntegerFieldType {
    #[serde(rename = "integer")]
    Integer,
}
impl From<&IntegerFieldType> for IntegerFieldType {
    fn from(value: &IntegerFieldType) -> Self {
        value.clone()
    }
}
impl ToString for IntegerFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Integer => "integer".to_string(),
        }
    }
}
impl std::str::FromStr for IntegerFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "integer" => Ok(Self::Integer),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for IntegerFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for IntegerFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for IntegerFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains numbers of any kind including decimals."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NumberField {
    #[doc = "a boolean field with a default of `true`. If `true` the physical contents of this field must follow the formatting constraints already set out. If `false` the contents of this field may contain leading and/or trailing non-numeric characters (which implementors MUST therefore strip). The purpose of `bareNumber` is to allow publishers to publish numeric data that contains trailing characters such as percentages e.g. `95%` or leading characters such as currencies e.g. `€95` or `EUR 95`. Note that it is entirely up to implementors what, if anything, they do with stripped text."]
    #[serde(rename = "bareNumber", default = "defaults::default_bool::<true>")]
    pub bare_number: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A string whose value is used to represent a decimal point within the number. The default value is `.`."]
    #[serde(
        rename = "decimalChar",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub decimal_char: Option<String>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `number`: only `default` is allowed."]
    #[serde(default = "defaults::number_field_format")]
    pub format: NumberFieldFormat,
    #[doc = "A string whose value is used to group digits within the number. The default value is `null`. A common value is `,` e.g. '100,000'."]
    #[serde(rename = "groupChar", default, skip_serializing_if = "Option::is_none")]
    pub group_char: Option<String>,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `number`."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<NumberFieldType>,
}
impl From<&NumberField> for NumberField {
    fn from(value: &NumberField) -> Self {
        value.clone()
    }
}
impl NumberField {
    pub fn builder() -> builder::NumberField {
        builder::NumberField::default()
    }
}
#[doc = "There are no format keyword options for `number`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NumberFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&NumberFieldFormat> for NumberFieldFormat {
    fn from(value: &NumberFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for NumberFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for NumberFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for NumberFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NumberFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NumberFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for NumberFieldFormat {
    fn default() -> Self {
        NumberFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `number`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum NumberFieldType {
    #[serde(rename = "number")]
    Number,
}
impl From<&NumberFieldType> for NumberFieldType {
    fn from(value: &NumberFieldType) -> Self {
        value.clone()
    }
}
impl ToString for NumberFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Number => "number".to_string(),
        }
    }
}
impl std::str::FromStr for NumberFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "number" => Ok(Self::Number),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for NumberFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NumberFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NumberFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains data which can be parsed as a valid JSON object."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ObjectField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `object`: only `default` is allowed."]
    #[serde(default = "defaults::object_field_format")]
    pub format: ObjectFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `object`."]
    #[serde(rename = "type")]
    pub type_: ObjectFieldType,
}
impl From<&ObjectField> for ObjectField {
    fn from(value: &ObjectField) -> Self {
        value.clone()
    }
}
impl ObjectField {
    pub fn builder() -> builder::ObjectField {
        builder::ObjectField::default()
    }
}
#[doc = "There are no format keyword options for `object`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ObjectFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&ObjectFieldFormat> for ObjectFieldFormat {
    fn from(value: &ObjectFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for ObjectFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for ObjectFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ObjectFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ObjectFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ObjectFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for ObjectFieldFormat {
    fn default() -> Self {
        ObjectFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `object`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ObjectFieldType {
    #[serde(rename = "object")]
    Object,
}
impl From<&ObjectFieldType> for ObjectFieldType {
    fn from(value: &ObjectFieldType) -> Self {
        value.clone()
    }
}
impl ToString for ObjectFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Object => "object".to_string(),
        }
    }
}
impl std::str::FromStr for ObjectFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "object" => Ok(Self::Object),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for ObjectFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for ObjectFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for ObjectFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "The field contains strings, that is, sequences of characters."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StringField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "The format keyword options for `string` are `default`, `email`, `uri`, `binary`, and `uuid`."]
    #[serde(default = "defaults::string_field_format")]
    pub format: StringFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `string`."]
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<StringFieldType>,
}
impl From<&StringField> for StringField {
    fn from(value: &StringField) -> Self {
        value.clone()
    }
}
impl StringField {
    pub fn builder() -> builder::StringField {
        builder::StringField::default()
    }
}
#[doc = "The format keyword options for `string` are `default`, `email`, `uri`, `binary`, and `uuid`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StringFieldFormat {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "uri")]
    Uri,
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "uuid")]
    Uuid,
}
impl From<&StringFieldFormat> for StringFieldFormat {
    fn from(value: &StringFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for StringFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
            Self::Email => "email".to_string(),
            Self::Uri => "uri".to_string(),
            Self::Binary => "binary".to_string(),
            Self::Uuid => "uuid".to_string(),
        }
    }
}
impl std::str::FromStr for StringFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            "email" => Ok(Self::Email),
            "uri" => Ok(Self::Uri),
            "binary" => Ok(Self::Binary),
            "uuid" => Ok(Self::Uuid),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StringFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StringFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StringFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for StringFieldFormat {
    fn default() -> Self {
        StringFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `string`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum StringFieldType {
    #[serde(rename = "string")]
    String,
}
impl From<&StringFieldType> for StringFieldType {
    fn from(value: &StringFieldType) -> Self {
        value.clone()
    }
}
impl ToString for StringFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::String => "string".to_string(),
        }
    }
}
impl std::str::FromStr for StringFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "string" => Ok(Self::String),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for StringFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for StringFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for StringFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "A Table Schema for this resource, compliant with the [Table Schema](/tableschema/) specification."]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
impl From<&TableSchema> for TableSchema {
    fn from(value: &TableSchema) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TableSchemaField {
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_0: Option<StringField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_1: Option<NumberField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_2: Option<IntegerField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_3: Option<DateField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_4: Option<TimeField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_5: Option<DateTimeField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_6: Option<YearField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_7: Option<YearMonthField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_8: Option<BooleanField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_9: Option<ObjectField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_10: Option<GeoPointField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_11: Option<GeoJsonField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_12: Option<ArrayField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_13: Option<DurationField>,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub subtype_14: Option<AnyField>,
}
impl From<&TableSchemaField> for TableSchemaField {
    fn from(value: &TableSchemaField) -> Self {
        value.clone()
    }
}
impl TableSchemaField {
    pub fn builder() -> builder::TableSchemaField {
        builder::TableSchemaField::default()
    }
}
#[doc = "A primary key is a field name or an array of field names, whose values `MUST` uniquely identify each row in the table."]
#[derive(Clone, Debug, Deserialize, Serialize)]
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
#[doc = "The field contains temporal time values."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "The format keyword options for `time` are `default`, `any`, and `{PATTERN}`."]
    #[serde(default = "defaults::time_field_format")]
    pub format: serde_json::Value,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `time`."]
    #[serde(rename = "type")]
    pub type_: TimeFieldType,
}
impl From<&TimeField> for TimeField {
    fn from(value: &TimeField) -> Self {
        value.clone()
    }
}
impl TimeField {
    pub fn builder() -> builder::TimeField {
        builder::TimeField::default()
    }
}
#[doc = "The type keyword, which `MUST` be a value of `time`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum TimeFieldType {
    #[serde(rename = "time")]
    Time,
}
impl From<&TimeFieldType> for TimeFieldType {
    fn from(value: &TimeFieldType) -> Self {
        value.clone()
    }
}
impl ToString for TimeFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Time => "time".to_string(),
        }
    }
}
impl std::str::FromStr for TimeFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "time" => Ok(Self::Time),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for TimeFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for TimeFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for TimeFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "A calendar year, being an integer with 4 digits. Equivalent to [gYear in XML Schema](https://www.w3.org/TR/xmlschema-2/#gYear)"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YearField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `year`: only `default` is allowed."]
    #[serde(default = "defaults::year_field_format")]
    pub format: YearFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `year`."]
    #[serde(rename = "type")]
    pub type_: YearFieldType,
}
impl From<&YearField> for YearField {
    fn from(value: &YearField) -> Self {
        value.clone()
    }
}
impl YearField {
    pub fn builder() -> builder::YearField {
        builder::YearField::default()
    }
}
#[doc = "There are no format keyword options for `year`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YearFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&YearFieldFormat> for YearFieldFormat {
    fn from(value: &YearFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for YearFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for YearFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for YearFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YearFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YearFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for YearFieldFormat {
    fn default() -> Self {
        YearFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `year`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YearFieldType {
    #[serde(rename = "year")]
    Year,
}
impl From<&YearFieldType> for YearFieldType {
    fn from(value: &YearFieldType) -> Self {
        value.clone()
    }
}
impl ToString for YearFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Year => "year".to_string(),
        }
    }
}
impl std::str::FromStr for YearFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "year" => Ok(Self::Year),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for YearFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YearFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YearFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "A calendar year month, being an integer with 1 or 2 digits. Equivalent to [gYearMonth in XML Schema](https://www.w3.org/TR/xmlschema-2/#gYearMonth)"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct YearMonthField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constraints: Option<Constraints>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "An example value for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub example: Option<String>,
    #[doc = "There are no format keyword options for `yearmonth`: only `default` is allowed."]
    #[serde(default = "defaults::year_month_field_format")]
    pub format: YearMonthFieldFormat,
    #[doc = "A name for this field."]
    pub name: String,
    #[doc = "The RDF type for this field."]
    #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
    pub rdf_type: Option<String>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "The type keyword, which `MUST` be a value of `yearmonth`."]
    #[serde(rename = "type")]
    pub type_: YearMonthFieldType,
}
impl From<&YearMonthField> for YearMonthField {
    fn from(value: &YearMonthField) -> Self {
        value.clone()
    }
}
impl YearMonthField {
    pub fn builder() -> builder::YearMonthField {
        builder::YearMonthField::default()
    }
}
#[doc = "There are no format keyword options for `yearmonth`: only `default` is allowed."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YearMonthFieldFormat {
    #[serde(rename = "default")]
    Default,
}
impl From<&YearMonthFieldFormat> for YearMonthFieldFormat {
    fn from(value: &YearMonthFieldFormat) -> Self {
        value.clone()
    }
}
impl ToString for YearMonthFieldFormat {
    fn to_string(&self) -> String {
        match *self {
            Self::Default => "default".to_string(),
        }
    }
}
impl std::str::FromStr for YearMonthFieldFormat {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "default" => Ok(Self::Default),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for YearMonthFieldFormat {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YearMonthFieldFormat {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YearMonthFieldFormat {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for YearMonthFieldFormat {
    fn default() -> Self {
        YearMonthFieldFormat::Default
    }
}
#[doc = "The type keyword, which `MUST` be a value of `yearmonth`."]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum YearMonthFieldType {
    #[serde(rename = "yearmonth")]
    Yearmonth,
}
impl From<&YearMonthFieldType> for YearMonthFieldType {
    fn from(value: &YearMonthFieldType) -> Self {
        value.clone()
    }
}
impl ToString for YearMonthFieldType {
    fn to_string(&self) -> String {
        match *self {
            Self::Yearmonth => "yearmonth".to_string(),
        }
    }
}
impl std::str::FromStr for YearMonthFieldType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "yearmonth" => Ok(Self::Yearmonth),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for YearMonthFieldType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for YearMonthFieldType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for YearMonthFieldType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct AnyField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::AnyFieldType, String>,
    }
    impl Default for AnyField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl AnyField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::AnyFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<AnyField> for super::AnyField {
        type Error = String;
        fn try_from(value: AnyField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::AnyField> for AnyField {
        fn from(value: super::AnyField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ArrayField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::ArrayFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::ArrayFieldType, String>,
    }
    impl Default for ArrayField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::array_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ArrayField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ArrayFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ArrayFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ArrayField> for super::ArrayField {
        type Error = String;
        fn try_from(value: ArrayField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ArrayField> for ArrayField {
        fn from(value: super::ArrayField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct BooleanField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        false_values: Result<Vec<String>, String>,
        format: Result<super::BooleanFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        true_values: Result<Vec<String>, String>,
        type_: Result<super::BooleanFieldType, String>,
    }
    impl Default for BooleanField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                false_values: Ok(super::defaults::boolean_field_false_values()),
                format: Ok(super::defaults::boolean_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                true_values: Ok(super::defaults::boolean_field_true_values()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl BooleanField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn false_values<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.false_values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for false_values: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BooleanFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn true_values<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.true_values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for true_values: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::BooleanFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<BooleanField> for super::BooleanField {
        type Error = String;
        fn try_from(value: BooleanField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                false_values: value.false_values?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                true_values: value.true_values?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::BooleanField> for BooleanField {
        fn from(value: super::BooleanField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                false_values: Ok(value.false_values),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                true_values: Ok(value.true_values),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Constraints {
        enum_: Result<Option<Vec<String>>, String>,
        max_length: Result<Option<i64>, String>,
        min_length: Result<Option<i64>, String>,
        pattern: Result<Option<String>, String>,
        required: Result<Option<bool>, String>,
        unique: Result<Option<bool>, String>,
    }
    impl Default for Constraints {
        fn default() -> Self {
            Self {
                enum_: Ok(Default::default()),
                max_length: Ok(Default::default()),
                min_length: Ok(Default::default()),
                pattern: Ok(Default::default()),
                required: Ok(Default::default()),
                unique: Ok(Default::default()),
            }
        }
    }
    impl Constraints {
        pub fn enum_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<Vec<String>>>,
            T::Error: std::fmt::Display,
        {
            self.enum_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for enum_: {}", e));
            self
        }
        pub fn max_length<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.max_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for max_length: {}", e));
            self
        }
        pub fn min_length<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.min_length = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_length: {}", e));
            self
        }
        pub fn pattern<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.pattern = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pattern: {}", e));
            self
        }
        pub fn required<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.required = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for required: {}", e));
            self
        }
        pub fn unique<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<bool>>,
            T::Error: std::fmt::Display,
        {
            self.unique = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unique: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Constraints> for super::Constraints {
        type Error = String;
        fn try_from(value: Constraints) -> Result<Self, String> {
            Ok(Self {
                enum_: value.enum_?,
                max_length: value.max_length?,
                min_length: value.min_length?,
                pattern: value.pattern?,
                required: value.required?,
                unique: value.unique?,
            })
        }
    }
    impl From<super::Constraints> for Constraints {
        fn from(value: super::Constraints) -> Self {
            Self {
                enum_: Ok(value.enum_),
                max_length: Ok(value.max_length),
                min_length: Ok(value.min_length),
                pattern: Ok(value.pattern),
                required: Ok(value.required),
                unique: Ok(value.unique),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DateField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<serde_json::Value, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::DateFieldType, String>,
    }
    impl Default for DateField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::date_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DateField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DateFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DateField> for super::DateField {
        type Error = String;
        fn try_from(value: DateField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DateField> for DateField {
        fn from(value: super::DateField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DateTimeField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<serde_json::Value, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::DateTimeFieldType, String>,
    }
    impl Default for DateTimeField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::date_time_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DateTimeField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DateTimeFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DateTimeField> for super::DateTimeField {
        type Error = String;
        fn try_from(value: DateTimeField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DateTimeField> for DateTimeField {
        fn from(value: super::DateTimeField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DurationField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::DurationFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::DurationFieldType, String>,
    }
    impl Default for DurationField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::duration_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl DurationField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DurationFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::DurationFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DurationField> for super::DurationField {
        type Error = String;
        fn try_from(value: DurationField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::DurationField> for DurationField {
        fn from(value: super::DurationField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeoJsonField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::GeoJsonFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::GeoJsonFieldType, String>,
    }
    impl Default for GeoJsonField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::geo_json_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GeoJsonField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GeoJsonFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GeoJsonFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<GeoJsonField> for super::GeoJsonField {
        type Error = String;
        fn try_from(value: GeoJsonField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GeoJsonField> for GeoJsonField {
        fn from(value: super::GeoJsonField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct GeoPointField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::GeoPointFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::GeoPointFieldType, String>,
    }
    impl Default for GeoPointField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::geo_point_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl GeoPointField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GeoPointFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::GeoPointFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<GeoPointField> for super::GeoPointField {
        type Error = String;
        fn try_from(value: GeoPointField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::GeoPointField> for GeoPointField {
        fn from(value: super::GeoPointField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IntegerField {
        bare_number: Result<bool, String>,
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::IntegerFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::IntegerFieldType, String>,
    }
    impl Default for IntegerField {
        fn default() -> Self {
            Self {
                bare_number: Ok(super::defaults::default_bool::<true>()),
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::integer_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl IntegerField {
        pub fn bare_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.bare_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bare_number: {}", e));
            self
        }
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IntegerFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::IntegerFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<IntegerField> for super::IntegerField {
        type Error = String;
        fn try_from(value: IntegerField) -> Result<Self, String> {
            Ok(Self {
                bare_number: value.bare_number?,
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::IntegerField> for IntegerField {
        fn from(value: super::IntegerField) -> Self {
            Self {
                bare_number: Ok(value.bare_number),
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct NumberField {
        bare_number: Result<bool, String>,
        constraints: Result<Option<super::Constraints>, String>,
        decimal_char: Result<Option<String>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::NumberFieldFormat, String>,
        group_char: Result<Option<String>, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<Option<super::NumberFieldType>, String>,
    }
    impl Default for NumberField {
        fn default() -> Self {
            Self {
                bare_number: Ok(super::defaults::default_bool::<true>()),
                constraints: Ok(Default::default()),
                decimal_char: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::number_field_format()),
                group_char: Ok(Default::default()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl NumberField {
        pub fn bare_number<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<bool>,
            T::Error: std::fmt::Display,
        {
            self.bare_number = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bare_number: {}", e));
            self
        }
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn decimal_char<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.decimal_char = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for decimal_char: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::NumberFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn group_char<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.group_char = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for group_char: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NumberFieldType>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<NumberField> for super::NumberField {
        type Error = String;
        fn try_from(value: NumberField) -> Result<Self, String> {
            Ok(Self {
                bare_number: value.bare_number?,
                constraints: value.constraints?,
                decimal_char: value.decimal_char?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                group_char: value.group_char?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::NumberField> for NumberField {
        fn from(value: super::NumberField) -> Self {
            Self {
                bare_number: Ok(value.bare_number),
                constraints: Ok(value.constraints),
                decimal_char: Ok(value.decimal_char),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                group_char: Ok(value.group_char),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ObjectField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::ObjectFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::ObjectFieldType, String>,
    }
    impl Default for ObjectField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::object_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl ObjectField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ObjectFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::ObjectFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<ObjectField> for super::ObjectField {
        type Error = String;
        fn try_from(value: ObjectField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::ObjectField> for ObjectField {
        fn from(value: super::ObjectField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct StringField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::StringFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<Option<super::StringFieldType>, String>,
    }
    impl Default for StringField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::string_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Ok(Default::default()),
            }
        }
    }
    impl StringField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::StringFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringFieldType>>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<StringField> for super::StringField {
        type Error = String;
        fn try_from(value: StringField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::StringField> for StringField {
        fn from(value: super::StringField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TableSchemaField {
        subtype_0: Result<Option<super::StringField>, String>,
        subtype_1: Result<Option<super::NumberField>, String>,
        subtype_2: Result<Option<super::IntegerField>, String>,
        subtype_3: Result<Option<super::DateField>, String>,
        subtype_4: Result<Option<super::TimeField>, String>,
        subtype_5: Result<Option<super::DateTimeField>, String>,
        subtype_6: Result<Option<super::YearField>, String>,
        subtype_7: Result<Option<super::YearMonthField>, String>,
        subtype_8: Result<Option<super::BooleanField>, String>,
        subtype_9: Result<Option<super::ObjectField>, String>,
        subtype_10: Result<Option<super::GeoPointField>, String>,
        subtype_11: Result<Option<super::GeoJsonField>, String>,
        subtype_12: Result<Option<super::ArrayField>, String>,
        subtype_13: Result<Option<super::DurationField>, String>,
        subtype_14: Result<Option<super::AnyField>, String>,
    }
    impl Default for TableSchemaField {
        fn default() -> Self {
            Self {
                subtype_0: Ok(Default::default()),
                subtype_1: Ok(Default::default()),
                subtype_2: Ok(Default::default()),
                subtype_3: Ok(Default::default()),
                subtype_4: Ok(Default::default()),
                subtype_5: Ok(Default::default()),
                subtype_6: Ok(Default::default()),
                subtype_7: Ok(Default::default()),
                subtype_8: Ok(Default::default()),
                subtype_9: Ok(Default::default()),
                subtype_10: Ok(Default::default()),
                subtype_11: Ok(Default::default()),
                subtype_12: Ok(Default::default()),
                subtype_13: Ok(Default::default()),
                subtype_14: Ok(Default::default()),
            }
        }
    }
    impl TableSchemaField {
        pub fn subtype_0<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::StringField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_0 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_0: {}", e));
            self
        }
        pub fn subtype_1<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::NumberField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_1 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_1: {}", e));
            self
        }
        pub fn subtype_2<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::IntegerField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_2 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_2: {}", e));
            self
        }
        pub fn subtype_3<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_3 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_3: {}", e));
            self
        }
        pub fn subtype_4<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TimeField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_4 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_4: {}", e));
            self
        }
        pub fn subtype_5<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DateTimeField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_5 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_5: {}", e));
            self
        }
        pub fn subtype_6<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::YearField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_6 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_6: {}", e));
            self
        }
        pub fn subtype_7<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::YearMonthField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_7 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_7: {}", e));
            self
        }
        pub fn subtype_8<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::BooleanField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_8 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_8: {}", e));
            self
        }
        pub fn subtype_9<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ObjectField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_9 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_9: {}", e));
            self
        }
        pub fn subtype_10<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GeoPointField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_10 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_10: {}", e));
            self
        }
        pub fn subtype_11<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::GeoJsonField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_11 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_11: {}", e));
            self
        }
        pub fn subtype_12<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::ArrayField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_12 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_12: {}", e));
            self
        }
        pub fn subtype_13<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::DurationField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_13 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_13: {}", e));
            self
        }
        pub fn subtype_14<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::AnyField>>,
            T::Error: std::fmt::Display,
        {
            self.subtype_14 = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subtype_14: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TableSchemaField> for super::TableSchemaField {
        type Error = String;
        fn try_from(value: TableSchemaField) -> Result<Self, String> {
            Ok(Self {
                subtype_0: value.subtype_0?,
                subtype_1: value.subtype_1?,
                subtype_2: value.subtype_2?,
                subtype_3: value.subtype_3?,
                subtype_4: value.subtype_4?,
                subtype_5: value.subtype_5?,
                subtype_6: value.subtype_6?,
                subtype_7: value.subtype_7?,
                subtype_8: value.subtype_8?,
                subtype_9: value.subtype_9?,
                subtype_10: value.subtype_10?,
                subtype_11: value.subtype_11?,
                subtype_12: value.subtype_12?,
                subtype_13: value.subtype_13?,
                subtype_14: value.subtype_14?,
            })
        }
    }
    impl From<super::TableSchemaField> for TableSchemaField {
        fn from(value: super::TableSchemaField) -> Self {
            Self {
                subtype_0: Ok(value.subtype_0),
                subtype_1: Ok(value.subtype_1),
                subtype_2: Ok(value.subtype_2),
                subtype_3: Ok(value.subtype_3),
                subtype_4: Ok(value.subtype_4),
                subtype_5: Ok(value.subtype_5),
                subtype_6: Ok(value.subtype_6),
                subtype_7: Ok(value.subtype_7),
                subtype_8: Ok(value.subtype_8),
                subtype_9: Ok(value.subtype_9),
                subtype_10: Ok(value.subtype_10),
                subtype_11: Ok(value.subtype_11),
                subtype_12: Ok(value.subtype_12),
                subtype_13: Ok(value.subtype_13),
                subtype_14: Ok(value.subtype_14),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct TimeField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<serde_json::Value, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::TimeFieldType, String>,
    }
    impl Default for TimeField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::time_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl TimeField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<serde_json::Value>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::TimeFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<TimeField> for super::TimeField {
        type Error = String;
        fn try_from(value: TimeField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::TimeField> for TimeField {
        fn from(value: super::TimeField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct YearField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::YearFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::YearFieldType, String>,
    }
    impl Default for YearField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::year_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl YearField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::YearFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::YearFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<YearField> for super::YearField {
        type Error = String;
        fn try_from(value: YearField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::YearField> for YearField {
        fn from(value: super::YearField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct YearMonthField {
        constraints: Result<Option<super::Constraints>, String>,
        description: Result<Option<String>, String>,
        example: Result<Option<String>, String>,
        format: Result<super::YearMonthFieldFormat, String>,
        name: Result<String, String>,
        rdf_type: Result<Option<String>, String>,
        title: Result<Option<String>, String>,
        type_: Result<super::YearMonthFieldType, String>,
    }
    impl Default for YearMonthField {
        fn default() -> Self {
            Self {
                constraints: Ok(Default::default()),
                description: Ok(Default::default()),
                example: Ok(Default::default()),
                format: Ok(super::defaults::year_month_field_format()),
                name: Err("no value supplied for name".to_string()),
                rdf_type: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl YearMonthField {
        pub fn constraints<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Constraints>>,
            T::Error: std::fmt::Display,
        {
            self.constraints = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for constraints: {}", e));
            self
        }
        pub fn description<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.description = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for description: {}", e));
            self
        }
        pub fn example<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.example = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for example: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::YearMonthFieldFormat>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn rdf_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.rdf_type = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rdf_type: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::YearMonthFieldType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<YearMonthField> for super::YearMonthField {
        type Error = String;
        fn try_from(value: YearMonthField) -> Result<Self, String> {
            Ok(Self {
                constraints: value.constraints?,
                description: value.description?,
                example: value.example?,
                format: value.format?,
                name: value.name?,
                rdf_type: value.rdf_type?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::YearMonthField> for YearMonthField {
        fn from(value: super::YearMonthField) -> Self {
            Self {
                constraints: Ok(value.constraints),
                description: Ok(value.description),
                example: Ok(value.example),
                format: Ok(value.format),
                name: Ok(value.name),
                rdf_type: Ok(value.rdf_type),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
}
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn array_field_format() -> super::ArrayFieldFormat {
        super::ArrayFieldFormat::Default
    }
    pub(super) fn boolean_field_false_values() -> Vec<String> {
        vec![
            "false".to_string(),
            "False".to_string(),
            "FALSE".to_string(),
            "0".to_string(),
        ]
    }
    pub(super) fn boolean_field_format() -> super::BooleanFieldFormat {
        super::BooleanFieldFormat::Default
    }
    pub(super) fn boolean_field_true_values() -> Vec<String> {
        vec![
            "true".to_string(),
            "True".to_string(),
            "TRUE".to_string(),
            "1".to_string(),
        ]
    }
    pub(super) fn date_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn date_time_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn duration_field_format() -> super::DurationFieldFormat {
        super::DurationFieldFormat::Default
    }
    pub(super) fn geo_json_field_format() -> super::GeoJsonFieldFormat {
        super::GeoJsonFieldFormat::Default
    }
    pub(super) fn geo_point_field_format() -> super::GeoPointFieldFormat {
        super::GeoPointFieldFormat::Default
    }
    pub(super) fn integer_field_format() -> super::IntegerFieldFormat {
        super::IntegerFieldFormat::Default
    }
    pub(super) fn number_field_format() -> super::NumberFieldFormat {
        super::NumberFieldFormat::Default
    }
    pub(super) fn object_field_format() -> super::ObjectFieldFormat {
        super::ObjectFieldFormat::Default
    }
    pub(super) fn string_field_format() -> super::StringFieldFormat {
        super::StringFieldFormat::Default
    }
    pub(super) fn table_schema_object_missing_values() -> Vec<String> {
        vec!["".to_string()]
    }
    pub(super) fn time_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn year_field_format() -> super::YearFieldFormat {
        super::YearFieldFormat::Default
    }
    pub(super) fn year_month_field_format() -> super::YearMonthFieldFormat {
        super::YearMonthFieldFormat::Default
    }
}
