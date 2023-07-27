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

use serde::{Deserialize, Serialize};

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
impl From<&TableSchema> for TableSchema {
    fn from(value: &TableSchema) -> Self {
        value.clone()
    }
}
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(untagged)]
pub enum TableSchemaField {
    StringField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "The format keyword options for `string` are `default`, `email`, `uri`, `binary`, and `uuid`."]
        #[serde(default = "defaults::table_schema_field_string_field_format")]
        format: StringFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `string`."]
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        type_: Option<StringFieldType>,
    },
    NumberField {
        #[doc = "a boolean field with a default of `true`. If `true` the physical contents of this field must follow the formatting constraints already set out. If `false` the contents of this field may contain leading and/or trailing non-numeric characters (which implementors MUST therefore strip). The purpose of `bareNumber` is to allow publishers to publish numeric data that contains trailing characters such as percentages e.g. `95%` or leading characters such as currencies e.g. `€95` or `EUR 95`. Note that it is entirely up to implementors what, if anything, they do with stripped text."]
        #[serde(rename = "bareNumber", default = "defaults::default_bool::<true>")]
        bare_number: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A string whose value is used to represent a decimal point within the number. The default value is `.`."]
        #[serde(
            rename = "decimalChar",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        decimal_char: Option<String>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `number`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_number_field_format")]
        format: NumberFieldFormat,
        #[doc = "A string whose value is used to group digits within the number. The default value is `null`. A common value is `,` e.g. '100,000'."]
        #[serde(rename = "groupChar", default, skip_serializing_if = "Option::is_none")]
        group_char: Option<String>,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `number`."]
        #[serde(rename = "type")]
        type_: NumberFieldType,
    },
    IntegerField {
        #[doc = "a boolean field with a default of `true`. If `true` the physical contents of this field must follow the formatting constraints already set out. If `false` the contents of this field may contain leading and/or trailing non-numeric characters (which implementors MUST therefore strip). The purpose of `bareNumber` is to allow publishers to publish numeric data that contains trailing characters such as percentages e.g. `95%` or leading characters such as currencies e.g. `€95` or `EUR 95`. Note that it is entirely up to implementors what, if anything, they do with stripped text."]
        #[serde(rename = "bareNumber", default = "defaults::default_bool::<true>")]
        bare_number: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `integer`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_integer_field_format")]
        format: IntegerFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `integer`."]
        #[serde(rename = "type")]
        type_: IntegerFieldType,
    },
    DateField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `date`."]
        #[serde(rename = "type")]
        type_: DateFieldType,
    },
    TimeField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `time`."]
        #[serde(rename = "type")]
        type_: TimeFieldType,
    },
    DateTimeField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `datetime`."]
        #[serde(rename = "type")]
        type_: DateTimeFieldType,
    },
    YearField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `year`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_year_field_format")]
        format: YearFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `year`."]
        #[serde(rename = "type")]
        type_: YearFieldType,
    },
    YearMonthField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `yearmonth`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_year_month_field_format")]
        format: YearMonthFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `yearmonth`."]
        #[serde(rename = "type")]
        type_: YearMonthFieldType,
    },
    BooleanField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[serde(
            rename = "falseValues",
            default = "defaults::table_schema_field_boolean_field_false_values"
        )]
        false_values: Vec<String>,
        #[doc = "There are no format keyword options for `boolean`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_boolean_field_format")]
        format: BooleanFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(
            rename = "trueValues",
            default = "defaults::table_schema_field_boolean_field_true_values"
        )]
        true_values: Vec<String>,
        #[doc = "The type keyword, which `MUST` be a value of `boolean`."]
        #[serde(rename = "type")]
        type_: BooleanFieldType,
    },
    ObjectField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `object`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_object_field_format")]
        format: ObjectFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `object`."]
        #[serde(rename = "type")]
        type_: ObjectFieldType,
    },
    GeoPointField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "The format keyword options for `geopoint` are `default`,`array`, and `object`."]
        #[serde(default = "defaults::table_schema_field_geo_point_field_format")]
        format: GeoPointFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `geopoint`."]
        #[serde(rename = "type")]
        type_: GeoPointFieldType,
    },
    GeoJsonField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "The format keyword options for `geojson` are `default` and `topojson`."]
        #[serde(default = "defaults::table_schema_field_geo_json_field_format")]
        format: GeoJsonFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `geojson`."]
        #[serde(rename = "type")]
        type_: GeoJsonFieldType,
    },
    ArrayField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `array`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_array_field_format")]
        format: ArrayFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `array`."]
        #[serde(rename = "type")]
        type_: ArrayFieldType,
    },
    DurationField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "There are no format keyword options for `duration`: only `default` is allowed."]
        #[serde(default = "defaults::table_schema_field_duration_field_format")]
        format: DurationFieldFormat,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `duration`."]
        #[serde(rename = "type")]
        type_: DurationFieldType,
    },
    AnyField {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        constraints: Option<Constraints>,
        #[doc = "A text description. Markdown is encouraged."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[doc = "An example value for the field."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        example: Option<String>,
        #[doc = "A name for this field."]
        name: String,
        #[doc = "The RDF type for this field."]
        #[serde(rename = "rdfType", default, skip_serializing_if = "Option::is_none")]
        rdf_type: Option<String>,
        #[doc = "A human-readable title."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[doc = "The type keyword, which `MUST` be a value of `any`."]
        #[serde(rename = "type")]
        type_: AnyFieldType,
    },
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
}
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn table_schema_field_array_field_format() -> super::ArrayFieldFormat {
        super::ArrayFieldFormat::Default
    }
    pub(super) fn table_schema_field_boolean_field_false_values() -> Vec<String> {
        vec![
            "false".to_string(),
            "False".to_string(),
            "FALSE".to_string(),
            "0".to_string(),
        ]
    }
    pub(super) fn table_schema_field_boolean_field_format() -> super::BooleanFieldFormat {
        super::BooleanFieldFormat::Default
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
    pub(super) fn table_schema_field_duration_field_format() -> super::DurationFieldFormat {
        super::DurationFieldFormat::Default
    }
    pub(super) fn table_schema_field_geo_json_field_format() -> super::GeoJsonFieldFormat {
        super::GeoJsonFieldFormat::Default
    }
    pub(super) fn table_schema_field_geo_point_field_format() -> super::GeoPointFieldFormat {
        super::GeoPointFieldFormat::Default
    }
    pub(super) fn table_schema_field_integer_field_format() -> super::IntegerFieldFormat {
        super::IntegerFieldFormat::Default
    }
    pub(super) fn table_schema_field_number_field_format() -> super::NumberFieldFormat {
        super::NumberFieldFormat::Default
    }
    pub(super) fn table_schema_field_object_field_format() -> super::ObjectFieldFormat {
        super::ObjectFieldFormat::Default
    }
    pub(super) fn table_schema_field_string_field_format() -> super::StringFieldFormat {
        super::StringFieldFormat::Default
    }
    pub(super) fn table_schema_field_time_field_format() -> serde_json::Value {
        serde_json::from_str::<serde_json::Value>("\"default\"").unwrap()
    }
    pub(super) fn table_schema_field_year_field_format() -> super::YearFieldFormat {
        super::YearFieldFormat::Default
    }
    pub(super) fn table_schema_field_year_month_field_format() -> super::YearMonthFieldFormat {
        super::YearMonthFieldFormat::Default
    }
    pub(super) fn table_schema_object_missing_values() -> Vec<String> {
        vec!["".to_string()]
    }
}
