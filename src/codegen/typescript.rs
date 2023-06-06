//! Typescript code generator.
use super::generator::{Generator, PackageDescriptor};
use super::DataPackage;
use convert_case::{Case, Casing};

pub struct TypeScript;

impl Generator for TypeScript {
    fn variable_name(self, name: &str) -> String {
        name.to_case(Case::Camel)
    }

    fn file_name(self, name: &str) -> String {
        format!("{}.ts", name.to_case(Case::Snake))
    }

    fn package_descriptor(self, _data_package: &DataPackage) -> PackageDescriptor {
        PackageDescriptor {
            file_name: String::from("package.json"),
            description: String::from("TODO: complete"),
        }
    }
}
