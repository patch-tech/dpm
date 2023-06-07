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
impl Contributor {
    pub fn builder() -> builder::Contributor {
        builder::Contributor::default()
    }
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
    pub id: Option<String>,
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
    pub resources: Vec<DataResource>,
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
impl DataPackage {
    pub fn builder() -> builder::DataPackage {
        builder::DataPackage::default()
    }
}
#[doc = "Data Resource."]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DataResource {
    #[doc = "The size of this resource in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<i64>,
    #[doc = "Inline data for this resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
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
    #[doc = "The media type of this resource. Can be any valid media type listed with [IANA](https://www.iana.org/assignments/media-types/media-types.xhtml)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mediatype: Option<MediaType>,
    #[doc = "An identifier string. Lower case characters with `.`, `_`, `-` and `/` are allowed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[doc = "A reference to the data for this resource, as either a path as a string, or an array of paths as strings. of valid URIs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<Path>,
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
impl DataResource {
    pub fn builder() -> builder::DataResource {
        builder::DataResource::default()
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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
impl License {
    pub fn builder() -> builder::License {
        builder::License::default()
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
        if regress::Regex::new("^([-a-z0-9._/])+$")
            .unwrap()
            .find(value)
            .is_none()
        {
            return Err("doesn't match pattern \"^([-a-z0-9._/])+$\"");
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
#[derive(Clone, Debug, Deserialize, Serialize)]
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
impl Source {
    pub fn builder() -> builder::Source {
        builder::Source::default()
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
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Contributor {
        email: Result<Option<String>, String>,
        organization: Result<Option<String>, String>,
        path: Result<Option<super::Path>, String>,
        role: Result<String, String>,
        title: Result<String, String>,
    }
    impl Default for Contributor {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                organization: Ok(Default::default()),
                path: Ok(Default::default()),
                role: Ok(super::defaults::contributor_role()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl Contributor {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {}", e));
            self
        }
        pub fn organization<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.organization = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for organization: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Path>>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn role<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.role = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for role: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Contributor> for super::Contributor {
        type Error = String;
        fn try_from(value: Contributor) -> Result<Self, String> {
            Ok(Self {
                email: value.email?,
                organization: value.organization?,
                path: value.path?,
                role: value.role?,
                title: value.title?,
            })
        }
    }
    impl From<super::Contributor> for Contributor {
        fn from(value: super::Contributor) -> Self {
            Self {
                email: Ok(value.email),
                organization: Ok(value.organization),
                path: Ok(value.path),
                role: Ok(value.role),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DataPackage {
        contributors: Result<Vec<super::Contributor>, String>,
        created: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
        description: Result<Option<String>, String>,
        homepage: Result<Option<String>, String>,
        id: Result<Option<String>, String>,
        image: Result<Option<String>, String>,
        keywords: Result<Vec<String>, String>,
        licenses: Result<Vec<super::License>, String>,
        name: Result<Option<super::Name>, String>,
        profile: Result<String, String>,
        resources: Result<Vec<super::DataResource>, String>,
        sources: Result<Vec<super::Source>, String>,
        title: Result<Option<String>, String>,
        version: Result<super::Version, String>,
    }
    impl Default for DataPackage {
        fn default() -> Self {
            Self {
                contributors: Ok(Default::default()),
                created: Ok(Default::default()),
                description: Ok(Default::default()),
                homepage: Ok(Default::default()),
                id: Ok(Default::default()),
                image: Ok(Default::default()),
                keywords: Ok(Default::default()),
                licenses: Ok(Default::default()),
                name: Ok(Default::default()),
                profile: Ok(super::defaults::data_package_profile()),
                resources: Err("no value supplied for resources".to_string()),
                sources: Ok(Default::default()),
                title: Ok(Default::default()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl DataPackage {
        pub fn contributors<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Contributor>>,
            T::Error: std::fmt::Display,
        {
            self.contributors = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contributors: {}", e));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
            T::Error: std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {}", e));
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
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for homepage: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn image<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.image = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for image: {}", e));
            self
        }
        pub fn keywords<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<String>>,
            T::Error: std::fmt::Display,
        {
            self.keywords = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for keywords: {}", e));
            self
        }
        pub fn licenses<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::License>>,
            T::Error: std::fmt::Display,
        {
            self.licenses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for licenses: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Name>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn profile<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profile: {}", e));
            self
        }
        pub fn resources<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::DataResource>>,
            T::Error: std::fmt::Display,
        {
            self.resources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for resources: {}", e));
            self
        }
        pub fn sources<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Source>>,
            T::Error: std::fmt::Display,
        {
            self.sources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sources: {}", e));
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
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::Version>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<DataPackage> for super::DataPackage {
        type Error = String;
        fn try_from(value: DataPackage) -> Result<Self, String> {
            Ok(Self {
                contributors: value.contributors?,
                created: value.created?,
                description: value.description?,
                homepage: value.homepage?,
                id: value.id?,
                image: value.image?,
                keywords: value.keywords?,
                licenses: value.licenses?,
                name: value.name?,
                profile: value.profile?,
                resources: value.resources?,
                sources: value.sources?,
                title: value.title?,
                version: value.version?,
            })
        }
    }
    impl From<super::DataPackage> for DataPackage {
        fn from(value: super::DataPackage) -> Self {
            Self {
                contributors: Ok(value.contributors),
                created: Ok(value.created),
                description: Ok(value.description),
                homepage: Ok(value.homepage),
                id: Ok(value.id),
                image: Ok(value.image),
                keywords: Ok(value.keywords),
                licenses: Ok(value.licenses),
                name: Ok(value.name),
                profile: Ok(value.profile),
                resources: Ok(value.resources),
                sources: Ok(value.sources),
                title: Ok(value.title),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct DataResource {
        bytes: Result<Option<i64>, String>,
        data: Result<Option<serde_json::Value>, String>,
        description: Result<Option<String>, String>,
        encoding: Result<Option<String>, String>,
        format: Result<Option<String>, String>,
        hash: Result<Option<super::Hash>, String>,
        homepage: Result<Option<String>, String>,
        licenses: Result<Vec<super::License>, String>,
        mediatype: Result<Option<super::MediaType>, String>,
        name: Result<Option<super::Name>, String>,
        path: Result<Option<super::Path>, String>,
        profile: Result<String, String>,
        schema: Result<Option<super::TableSchema>, String>,
        sources: Result<Vec<super::Source>, String>,
        title: Result<Option<String>, String>,
    }
    impl Default for DataResource {
        fn default() -> Self {
            Self {
                bytes: Ok(Default::default()),
                data: Ok(Default::default()),
                description: Ok(Default::default()),
                encoding: Ok(Default::default()),
                format: Ok(Default::default()),
                hash: Ok(Default::default()),
                homepage: Ok(Default::default()),
                licenses: Ok(Default::default()),
                mediatype: Ok(Default::default()),
                name: Ok(Default::default()),
                path: Ok(Default::default()),
                profile: Ok(super::defaults::data_resource_profile()),
                schema: Ok(Default::default()),
                sources: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl DataResource {
        pub fn bytes<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<i64>>,
            T::Error: std::fmt::Display,
        {
            self.bytes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for bytes: {}", e));
            self
        }
        pub fn data<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<serde_json::Value>>,
            T::Error: std::fmt::Display,
        {
            self.data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for data: {}", e));
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
        pub fn encoding<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.encoding = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for encoding: {}", e));
            self
        }
        pub fn format<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.format = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for format: {}", e));
            self
        }
        pub fn hash<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Hash>>,
            T::Error: std::fmt::Display,
        {
            self.hash = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hash: {}", e));
            self
        }
        pub fn homepage<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.homepage = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for homepage: {}", e));
            self
        }
        pub fn licenses<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::License>>,
            T::Error: std::fmt::Display,
        {
            self.licenses = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for licenses: {}", e));
            self
        }
        pub fn mediatype<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::MediaType>>,
            T::Error: std::fmt::Display,
        {
            self.mediatype = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for mediatype: {}", e));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Name>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Path>>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn profile<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.profile = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for profile: {}", e));
            self
        }
        pub fn schema<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::TableSchema>>,
            T::Error: std::fmt::Display,
        {
            self.schema = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for schema: {}", e));
            self
        }
        pub fn sources<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Vec<super::Source>>,
            T::Error: std::fmt::Display,
        {
            self.sources = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for sources: {}", e));
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
    }
    impl std::convert::TryFrom<DataResource> for super::DataResource {
        type Error = String;
        fn try_from(value: DataResource) -> Result<Self, String> {
            Ok(Self {
                bytes: value.bytes?,
                data: value.data?,
                description: value.description?,
                encoding: value.encoding?,
                format: value.format?,
                hash: value.hash?,
                homepage: value.homepage?,
                licenses: value.licenses?,
                mediatype: value.mediatype?,
                name: value.name?,
                path: value.path?,
                profile: value.profile?,
                schema: value.schema?,
                sources: value.sources?,
                title: value.title?,
            })
        }
    }
    impl From<super::DataResource> for DataResource {
        fn from(value: super::DataResource) -> Self {
            Self {
                bytes: Ok(value.bytes),
                data: Ok(value.data),
                description: Ok(value.description),
                encoding: Ok(value.encoding),
                format: Ok(value.format),
                hash: Ok(value.hash),
                homepage: Ok(value.homepage),
                licenses: Ok(value.licenses),
                mediatype: Ok(value.mediatype),
                name: Ok(value.name),
                path: Ok(value.path),
                profile: Ok(value.profile),
                schema: Ok(value.schema),
                sources: Ok(value.sources),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct License {
        name: Result<Option<super::OpenDefinitionLicenseIdentifier>, String>,
        path: Result<Option<super::Path>, String>,
        title: Result<Option<String>, String>,
    }
    impl Default for License {
        fn default() -> Self {
            Self {
                name: Ok(Default::default()),
                path: Ok(Default::default()),
                title: Ok(Default::default()),
            }
        }
    }
    impl License {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::OpenDefinitionLicenseIdentifier>>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Path>>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
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
    }
    impl std::convert::TryFrom<License> for super::License {
        type Error = String;
        fn try_from(value: License) -> Result<Self, String> {
            Ok(Self {
                name: value.name?,
                path: value.path?,
                title: value.title?,
            })
        }
    }
    impl From<super::License> for License {
        fn from(value: super::License) -> Self {
            Self {
                name: Ok(value.name),
                path: Ok(value.path),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Source {
        email: Result<Option<String>, String>,
        path: Result<Option<super::Path>, String>,
        title: Result<String, String>,
    }
    impl Default for Source {
        fn default() -> Self {
            Self {
                email: Ok(Default::default()),
                path: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl Source {
        pub fn email<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.email = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for email: {}", e));
            self
        }
        pub fn path<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::Path>>,
            T::Error: std::fmt::Display,
        {
            self.path = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for path: {}", e));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<Source> for super::Source {
        type Error = String;
        fn try_from(value: Source) -> Result<Self, String> {
            Ok(Self {
                email: value.email?,
                path: value.path?,
                title: value.title?,
            })
        }
    }
    impl From<super::Source> for Source {
        fn from(value: super::Source) -> Self {
            Self {
                email: Ok(value.email),
                path: Ok(value.path),
                title: Ok(value.title),
            }
        }
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
