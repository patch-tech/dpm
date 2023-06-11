mod generator;
mod typescript;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use super::command::Target;
use super::descriptor::DataPackage;
pub use generator::Generator;
pub use typescript::TypeScript;

/// ItemRef stores the name of a generated item, such as a Class or variable,
/// and the filename that contains its definition.
struct ItemRef {
    ref_name: String,
    path: String,
}

/// Outputs all static assets to the output directory.
fn output_static_assets(generator: &impl Generator, output: &Path) {
    for static_asset in generator.static_assets() {
        let target = output.join(&static_asset.path);
        let parent = target.parent().unwrap();
        let create_dir_res = fs::create_dir_all(parent);
        if create_dir_res.is_err() {
            panic!(
                "Failed to create parent directories for {:?}, with error {:?}",
                target,
                create_dir_res.err()
            );
        }

        match fs::write(&target, static_asset.content.data) {
            Ok(_) => println!("Copied asset {:?} to {:?}", static_asset.path, target),
            Err(e) => panic!(
                "Failed to copy asset {:?} to {:?}, with error {e}",
                static_asset.path, target
            ),
        }
    }
}

/// Outputs all generated table definitions, one per resource, to the output directory.
/// Returns the item references for each generated definition.
/// The table definition will use the particular target language's feature,
/// e.g., Class in TypeScript, Python, Ruby; Struct in Rust, Golang.
fn output_table_definitions(generator: &impl Generator, output: &Path) -> Vec<ItemRef> {
    let dp = generator.data_package();
    let mut item_refs: Vec<ItemRef> = Vec::new();
    let mut names_seen: HashSet<String> = HashSet::new();
    for r in &dp.resources {
        let asset = generator.resource_table(r);
        if names_seen.contains(&asset.name) {
            panic!("Duplicate table definition found {:?}", asset.name);
        }
        names_seen.insert(asset.name.to_string());

        let asset_path = &asset.path;
        let target = output.join(asset_path);
        match fs::write(&target, asset.content) {
            Err(e) => panic!(
                "Failed to write table definition {:?} with error: {:?}",
                asset.name, e
            ),
            _ => println!(
                "Wrote table definition {:?} for resource {:?} to {:?}",
                asset.name,
                r.name.as_ref().unwrap(),
                target
            ),
        }

        item_refs.push(ItemRef {
            ref_name: asset.name,
            path: asset.path,
        });
    }
    item_refs
}

pub fn generate_package(dp: &DataPackage, target: &Target, output: &Path) -> () {
    println!("Going to generate a data-package in {:?}", target);
    let generator = target.generator_for_package(dp);

    let out_root_dir = output.join(generator.root_dir());
    let out_src_dir = out_root_dir.join(generator.source_dir());

    // PAT-3370: Output static files.
    output_static_assets(&generator, &out_root_dir);

    // PAT-3369: Generate and output table definitions for each resource.
    let _table_definitions = output_table_definitions(&generator, &out_src_dir);

    // TODO(PAT-3370): Output dataset, entry-point files.
    // TODO(PAT-3370): Output package descriptor.
}
