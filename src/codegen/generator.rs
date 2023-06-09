//! Code generator trait.

use rust_embed::EmbeddedFile;

use crate::descriptor::{DataPackage, DataResource};

/// PackageDescriptor describes a particular language's package descriptor.
/// E.g., for `TypeScript`, we use
/// ```ignore
/// PackageDescriptpr {
///   file_name: "package.json",
///   description: "<json description of package>"
/// }
/// ```
pub struct PackageDescriptor {
    pub file_name: String,
    pub description: String,
}

pub struct StaticAsset {
    pub path: String,
    pub content: EmbeddedFile,
}

// A dynamic asset represents any generated code item, e.g. a class, a variable.
pub struct DynamicAsset {
    /// Location of asset, typically a file name.
    pub path: String,
    /// Name of generated asset, typically a class name.
    pub name: String,
    /// Definition of asset, e.g. code that defines a class.
    pub content: String,
}

pub trait Generator {
    /// The data package that the generator is processing.
    fn data_package(&self) -> &DataPackage;

    /// Returns a dynamic asset that represents a generated table class
    /// corresponding to the resource.
    fn resource_table(&self, r: &DataResource) -> DynamicAsset;

    /// The current version of the language's static code.
    fn version(&self) -> String;

    /// Returns a vector of static assets.
    fn static_assets(&self) -> Vec<StaticAsset>;

    /// The entry file name for the language.
    fn entry_file_name(&self) -> String;

    /// The root directory for the language.
    fn root_dir(&self) -> String;

    /// The source directory for the language.
    fn source_dir(&self) -> String;

    /// Returns a variable name in the language, given a name.
    fn variable_name(&self, name: &str) -> String;

    /// Returns a file name in the language, given a name.
    fn file_name(&self, name: &str) -> String;

    /// Returns a package name in the language, given a name.
    fn package_name(&self, name: &str) -> String;

    /// Returns a package descriptor in the language.
    fn package_descriptor(&self) -> PackageDescriptor;
}
