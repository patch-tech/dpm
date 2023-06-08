//! Typescript code generator.

use super::generator::{Generator, PackageDescriptor, StaticAsset};
use super::DataPackage;
use convert_case::{Case, Casing};
use rust_embed::RustEmbed;

pub struct TypeScript<'a> {
    pub data_package: &'a DataPackage,
}

const TYPESCRIPT_VERSION: &str = "0.1.0";

#[derive(RustEmbed)]
#[folder = "static/typescript/0.1.0/"]
struct Asset;

impl Generator for TypeScript<'_> {
    fn version(&self) -> String {
        String::from(TYPESCRIPT_VERSION)
    }

    fn static_assets(&self) -> Vec<StaticAsset> {
        Asset::iter()
            .map(|p| StaticAsset {
                path: p.to_string(),
                asset: Asset::get(&p).unwrap(),
            })
            .collect()
    }

    fn entry_file_name(&self) -> String {
        String::from("index.ts")
    }

    fn root_dir(&self) -> String {
        String::from("typescript")
    }

    fn source_dir(&self) -> String {
        String::from("")
    }

    fn variable_name(&self, name: &str) -> String {
        name.to_case(Case::Camel)
    }

    fn file_name(&self, name: &str) -> String {
        format!("{}.ts", name.to_case(Case::Snake))
    }

    fn package_name(&self, name: &str) -> String {
        name.to_case(Case::Kebab)
    }

    fn package_descriptor(&self) -> PackageDescriptor {
        PackageDescriptor {
            file_name: String::from("package.json"),
            description: String::from("TODO: complete"),
        }
    }
}
