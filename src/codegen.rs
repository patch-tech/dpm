mod generator;
mod nodejs;
mod python;

use dialoguer::Confirm;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use clap::Subcommand;

pub use generator::{Generator, ItemRef};
pub use nodejs::NodeJs;
pub use python::Python;

use crate::api::GetPackageVersionResponse;

#[derive(Subcommand, Debug)]
pub enum Target {
    /// Build a Node.js data package
    #[command(name = "nodejs")]
    NodeJs {
        #[arg(short, long)]
        scope: Option<String>,
    },

    /// Build a Python data package
    Python,
}

impl Target {
    pub fn generator_for_package<'a>(
        &self,
        dp: &'a GetPackageVersionResponse,
    ) -> Box<dyn Generator + 'a> {
        let generator: Box<dyn Generator> = match self {
            Target::NodeJs { scope } => Box::new(NodeJs::new(dp, scope.clone())),
            Target::Python {} => Box::new(Python::new(dp)),
        };
        generator
    }
}

fn write<C: AsRef<[u8]>>(target: &Path, content: C, msg_snippet: String) {
    let parent = target.parent().unwrap();
    let create_dir_res = fs::create_dir_all(parent);
    if create_dir_res.is_err() {
        panic!(
            "Failed to create parent directories for {:?}, with error {:?}",
            target,
            create_dir_res.err()
        );
    }
    match fs::write(target, content) {
        Ok(_) => println!("Wrote {msg_snippet} to {:?}", target),
        Err(e) => panic!(
            "Failed to write {msg_snippet} to {:?}, with error {e}",
            target
        ),
    }
}

fn check_package_existence(path: &PathBuf, assume_yes: bool) {
    if path.exists() {
        if assume_yes
            || Confirm::new()
                .with_prompt(format!(
                    "Data package already exists at {:?}, overwrite?",
                    path
                ))
                .interact()
                .unwrap()
        {
            println!("Overwriting");
        } else {
            println!("Package generation cancelled");
            process::exit(1);
        }
    }
}

/// Outputs all static assets to the output directory. These assets are
/// typically code that defines basic types, such as `Field`, `Table`, which are
/// used to define the specific resources present in the datapackage.json.
fn output_static_assets(generator: &dyn Generator, output: &Path) {
    for static_asset in generator.static_assets() {
        let target = output.join(static_asset.path.as_path());
        write(
            &target,
            &static_asset.content.data,
            format!("asset {:?}", static_asset.path),
        );
    }
}

/// Outputs all generated table definitions, one per resource, to the output directory.
/// Returns the item references for each generated definition.
/// The table definition will use the particular target language's feature,
/// e.g., Class in TypeScript, Python, Ruby; Struct in Rust, Golang.
fn output_table_definitions(generator: &dyn Generator, output: &Path) -> Vec<ItemRef> {
    let dp = generator.data_package();
    let mut item_refs: Vec<ItemRef> = Vec::new();
    let mut names_seen: HashSet<String> = HashSet::new();
    for r in &dp.version.dataset {
        let asset = generator.resource_table(r);
        if names_seen.contains(&asset.name) {
            panic!("Duplicate table definition found {:?}", asset.name);
        }
        names_seen.insert(asset.name.to_string());

        let asset_path = &asset.path;
        let target = output.join(asset_path.as_path());
        write(
            &target,
            asset.content,
            format!(
                "table definition {:?} for resource {:?}",
                asset.name, r.name
            ),
        );

        item_refs.push(ItemRef {
            ref_name: asset.name,
            path: asset.path,
        });
    }
    item_refs
}

/// Outputs the manifest for the generated data package code.
fn output_manifest(generator: &dyn Generator, output: &Path) {
    let manifest = generator.manifest();
    let target = output.join(manifest.file_name);
    write(&target, manifest.description, "manifest".to_string());
}

/// Outputs a file containing the code version of the data package.
fn output_version(generator: &dyn Generator, output: &Path) {
    let version = generator.version();
    let target = output.join(version.path.as_path());
    write(&target, version.content, "version".to_string());
}

/// Outputs the entry point for the generated data package code. E.g., for
/// Node.js this is the `index.ts` file containing the table exports.
fn output_entry_point(generator: &dyn Generator, table_definitions: Vec<ItemRef>, output: &Path) {
    let entry_code = generator.entry_code(table_definitions);
    let target = output.join(entry_code.path.as_path());
    write(&target, entry_code.content, "entry code".to_string());
}

pub fn generate_package(
    dp: &GetPackageVersionResponse,
    target: &Target,
    output: &Path,
    assume_yes: bool,
) {
    println!("Going to generate a data-package in {:?}", target);
    let generator = target.generator_for_package(dp);

    let out_root_dir = output.join(generator.root_dir());
    check_package_existence(&out_root_dir, assume_yes);
    output_static_assets(generator.as_ref(), &out_root_dir);
    let table_definitions = output_table_definitions(generator.as_ref(), &out_root_dir);
    output_version(generator.as_ref(), &out_root_dir);
    output_entry_point(generator.as_ref(), table_definitions, &out_root_dir);
    output_manifest(generator.as_ref(), &out_root_dir);
    generator.build_package(&out_root_dir);
}
