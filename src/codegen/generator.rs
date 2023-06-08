//! Code generator trait.

use rust_embed::EmbeddedFile;

/// PackageDescriptor describes a particular language's package descriptor.
/// E.g., for `TypeScript`, we use
/// ```rust
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
    pub asset: EmbeddedFile,
}

pub trait Generator {
    /// The current version of the language's static code.
    fn version(&self) -> String;

    /// Returns an iterator of static assets.
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
