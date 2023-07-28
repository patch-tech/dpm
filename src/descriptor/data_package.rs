//! Generated with https://github.com/oxidecomputer/typify/tree/v0.0.13 from the
//! JSON Schema at https://specs.frictionlessdata.io/data-package/, accessed on
//! 2023-06-04. On that day the page at that URL said "Updated 2 May 2017".

#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(dead_code)]

use super::table_schema::TableSchema;
use serde::{Deserialize, Serialize};
use uuid7::Uuid;

#[doc = "A contributor to this descriptor."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Contributor {
    #[doc = "An email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "An organizational affiliation for this contributor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    #[doc = "A fully qualified URL, or a POSIX file path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
    #[serde(default = "defaults::contributor_role")]
    pub role: String,
    #[doc = "A human-readable title."]
    pub title: String,
}
impl From<&Contributor> for Contributor {
    fn from(value: &Contributor) -> Self {
        value.clone()
    }
}

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
#[doc = "Data Package is a simple specification for data access and delivery."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataPackage {
    #[doc = "The contributors to this descriptor."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contributors: Vec<Contributor>,
    #[doc = "The datetime on which this descriptor was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The home on the web that is related to this data package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[doc = "A property reserved for globally unique identifiers. Examples of identifiers that are unique include UUIDs and DOIs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>,
    #[doc = "A image to represent this package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[doc = "A list of keywords that describe this package."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
    #[doc = "The license(s) under which this package is published."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub licenses: Vec<License>,
    #[doc = "An identifier string. Lower case characters with `.`, `_`, `-` and `/` are allowed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[doc = "The profile of this descriptor."]
    #[serde(default = "defaults::data_package_profile")]
    pub profile: String,
    #[doc = "An `array` of Data Resource objects, each compliant with the [Data Resource](/data-resource/) specification."]
    pub dataset: Vec<DataResource>,
    #[doc = "The raw sources for this resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<Source>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[doc = "A semantic version."]
    pub version: Version,
}
impl From<&DataPackage> for DataPackage {
    fn from(value: &DataPackage) -> Self {
        value.clone()
    }
}
#[doc = "Data Resource."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct DataResource {
    #[doc = "The size of this resource in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[doc = "A text description. Markdown is encouraged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The file encoding of this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[doc = "The file format of this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "The MD5 hash of this resource. Indicate other hashing algorithms with the {algorithm}:{hash} format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<Hash>,
    #[doc = "The home on the web that is related to this data package."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    #[doc = "The license(s) under which the resource is published."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub licenses: Vec<License>,
    #[doc = "Where the table data resides"]
    pub location: TableLocation,
    #[doc = "The media type of this resource. Can be any valid media type listed with [IANA](https://www.iana.org/assignments/media-types/media-types.xhtml)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mediatype: Option<MediaType>,
    #[doc = "An identifier string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A reference to the data for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The profile of this descriptor."]
    #[serde(default = "defaults::data_resource_profile")]
    pub profile: String,
    #[doc = "A schema for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<TableSchema>,
    #[doc = "The raw sources for this resource."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<Source>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl From<&DataResource> for DataResource {
    fn from(value: &DataResource) -> Self {
        value.clone()
    }
}
#[doc = "The MD5 hash of this resource. Indicate other hashing algorithms with the {algorithm}:{hash} format."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Hash(String);
impl std::ops::Deref for Hash {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Hash> for String {
    fn from(value: Hash) -> Self {
        value.0
    }
}
impl From<&Hash> for Hash {
    fn from(value: &Hash) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Hash {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^([^:]+:[a-fA-F0-9]+|[a-fA-F0-9]{32}|)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^([^:]+:[a-fA-F0-9]+|[a-fA-F0-9]{32}|)$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Hash {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Hash {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Hash {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A license for this descriptor."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct License {
    #[doc = "MUST be an Open Definition license identifier, see http://licenses.opendefinition.org/"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<OpenDefinitionLicenseIdentifier>,
    #[doc = "A fully qualified URL, or a POSIX file path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
    #[doc = "A human-readable title."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}
impl From<&License> for License {
    fn from(value: &License) -> Self {
        value.clone()
    }
}
#[doc = "The media type of this resource. Can be any valid media type listed with [IANA](https://www.iana.org/assignments/media-types/media-types.xhtml)."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MediaType(String);
impl std::ops::Deref for MediaType {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<MediaType> for String {
    fn from(value: MediaType) -> Self {
        value.0
    }
}
impl From<&MediaType> for MediaType {
    fn from(value: &MediaType) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for MediaType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^(.+)/(.+)$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(.+)/(.+)$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for MediaType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for MediaType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for MediaType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "An identifier string. Lower case characters with `.`, `_`, `-` and `/` are allowed."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Name(String);
impl std::ops::Deref for Name {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Name> for String {
    fn from(value: Name) -> Self {
        value.0
    }
}
impl From<&Name> for Name {
    fn from(value: &Name) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Name {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^([-A-Za-z0-9._/])+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^([-A-Za-z0-9._/])+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Name {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Name {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Name {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "MUST be an Open Definition license identifier, see http://licenses.opendefinition.org/"]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct OpenDefinitionLicenseIdentifier(String);
impl std::ops::Deref for OpenDefinitionLicenseIdentifier {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<OpenDefinitionLicenseIdentifier> for String {
    fn from(value: OpenDefinitionLicenseIdentifier) -> Self {
        value.0
    }
}
impl From<&OpenDefinitionLicenseIdentifier> for OpenDefinitionLicenseIdentifier {
    fn from(value: &OpenDefinitionLicenseIdentifier) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for OpenDefinitionLicenseIdentifier {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^([-a-zA-Z0-9._])+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^([-a-zA-Z0-9._])+$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for OpenDefinitionLicenseIdentifier {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for OpenDefinitionLicenseIdentifier {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for OpenDefinitionLicenseIdentifier {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for OpenDefinitionLicenseIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A fully qualified URL, or a POSIX file path."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Path(String);
impl std::ops::Deref for Path {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Path> for String {
    fn from(value: Path) -> Self {
        value.0
    }
}
impl From<&Path> for Path {
    fn from(value: &Path) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Path {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress::Regex::new("^(?=^[^./~])(^((?!\\.{2}).)*$).*$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^(?=^[^./~])(^((?!\\.{2}).)*$).*$\"");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Path {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Path {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Path {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "A source file."]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Source {
    #[doc = "An email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "A fully qualified URL, or a POSIX file path."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
    #[doc = "A human-readable title."]
    pub title: String,
}
impl From<&Source> for Source {
    fn from(value: &Source) -> Self {
        value.clone()
    }
}
#[doc = "A semantic version."]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Version(String);
impl std::ops::Deref for Version {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<Version> for String {
    fn from(value: Version) -> Self {
        value.0
    }
}
impl From<&Version> for Version {
    fn from(value: &Version) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for Version {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if regress :: Regex :: new ("^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$") . unwrap () . find (value) . is_none () { return Err ("doesn't match pattern \"^(0|[1-9]\\d*)\\.(0|[1-9]\\d*)\\.(0|[1-9]\\d*)(?:-((?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\\.(?:0|[1-9]\\d*|\\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\\+([0-9a-zA-Z-]+(?:\\.[0-9a-zA-Z-]+)*))?$\"") ; }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for Version {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Version {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Version {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
pub mod defaults {
    pub(super) fn contributor_role() -> String {
        "contributor".to_string()
    }
    pub(super) fn data_package_profile() -> String {
        "data-package".to_string()
    }
    pub(super) fn data_resource_profile() -> String {
        "data-resource".to_string()
    }
}
