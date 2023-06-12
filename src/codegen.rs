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

fn write<C: AsRef<[u8]>>(target: &Path, content: C, msg_snippet: String) {
    match fs::write(&target, content) {
        Ok(_) => println!("Wrote {msg_snippet} to {:?}", target),
        Err(e) => panic!(
            "Failed to write {msg_snippet} to {:?}, with error {e}",
            target
        ),
    }
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
        write(
            &target,
            asset.content,
            format!(
                "table definition {:?} for resource {:?}",
                asset.name,
                r.name.as_ref().unwrap()
            ),
        );

        item_refs.push(ItemRef {
            ref_name: asset.name,
            path: asset.path,
        });
    }
    item_refs
}

fn output_dataset_definition(generator: &impl Generator, output: &Path) {
    let asset = generator.dataset_definition();

    let asset_path = &asset.path;
    let target = output.join(asset_path);
    write(
        &target,
        asset.content,
        format!("dataset definition {:?}", asset.name),
    );
}

/// Outputs the manifest for the generated data package code.
fn output_manifest(generator: &impl Generator, output: &Path) {
    let manifest = generator.manifest();
    let target = output.join(manifest.file_name);
    write(&target, manifest.description, "manifest".to_string());
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

    // PAT-3464: Output dataset, entry-point files.
    output_dataset_definition(&generator, &out_src_dir);
    output_manifest(&generator, &out_root_dir);
}
