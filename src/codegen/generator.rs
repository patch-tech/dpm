//! Code generator trait.

use rust_embed::EmbeddedFile;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::api::GetDatasetVersionResponse;
use crate::descriptor::Table;

/// ItemRef stores the name of a generated item, such as a Class or variable,
/// and the filename that contains its definition.
#[derive(Serialize)]
pub struct ItemRef {
    pub ref_name: String,
    pub path: Box<PathBuf>,
}

/// Manifest describes a particular language's descriptor for an installable code package.
/// E.g., for `Node.js`, we use
/// ```ignore
/// Manifest {
///   file_name: "package.json",
///   description: "<json description of package>"
/// }
/// ```
pub struct Manifest {
    pub file_name: String,
    pub description: String,
}

pub struct StaticAsset {
    pub path: Box<PathBuf>,
    pub content: EmbeddedFile,
}

// A dynamic asset represents any generated code item, e.g. a class, a variable.
pub struct DynamicAsset {
    /// Location of asset, typically a file name.
    pub path: Box<PathBuf>,
    /// Name of generated asset, typically a class name.
    pub name: String,
    /// Definition of asset, e.g. code that defines a class.
    pub content: String,
}

/// Executes a command with the given path as the working directory.
pub fn exec_cmd(name: &str, path: &Path, cmd: &str, args: &[&str]) {
    let mut cmd = Command::new(cmd);
    cmd.current_dir(path);
    cmd.args(args);
    let output = cmd.output().expect("Failed to {name}");
    if !output.status.success() {
        panic!("Failed to {name} with error {:?}", output.stderr);
    }
}

/// A type that derives the contents of a data package from a `Dataset`.
pub trait Generator {
    /// The dataset that the generator is processing.
    fn dataset(&self) -> &GetDatasetVersionResponse;

    /// Returns a dynamic asset that represents a generated table definition
    /// corresponding to the resource.
    fn resource_table(&self, r: &Table) -> DynamicAsset;

    /// Returns a dynamic asset that contains the current version of the
    /// language's static code.
    fn version(&self) -> DynamicAsset;

    /// Returns static assets produced by this generator.
    fn static_assets(&self) -> Vec<StaticAsset>;

    /// The entry file name for the language.
    fn entry_file_name(&self) -> String;

    /// The root directory for the language.
    fn root_dir(&self) -> PathBuf;

    /// The source directory for the language.
    fn source_dir(&self) -> String;

    /// Returns a variable name in the language, given a name.
    fn variable_name(&self, name: &str) -> String;

    /// Returns a file name in the language, given a name.
    fn file_name(&self, name: &str) -> String;

    /// Returns a package name in the language, given a name.
    fn dataset_name(&self, name: &str) -> String;

    /// Returns a manifest used by the language.
    fn manifest(&self) -> Manifest;

    /// Returns entry code definition for the target. E.g. for `Node.js`
    /// returns the contents of an `index.ts` file.
    fn entry_code(&self, imports: Vec<ItemRef>) -> DynamicAsset;

    fn build_package(&self, output: &Path);
}
