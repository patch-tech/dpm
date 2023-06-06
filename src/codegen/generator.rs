//! Code generator trait.
use super::DataPackage;

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

pub trait Generator {
    /// Returns a variable name in the language, given a name.
    fn variable_name(self, name: &str) -> String;

    /// Returns a file name in the language, given a name.
    fn file_name(self, name: &str) -> String;

    /// Returns a package descriptor in the language.
    fn package_descriptor(self, data_package: &DataPackage) -> PackageDescriptor;
}
