mod generator;
mod typescript;

use std::fs;
use std::path::Path;

use super::command::Target;
use super::descriptor::DataPackage;
pub use generator::Generator;
pub use typescript::TypeScript;

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

        match fs::write(&target, static_asset.asset.data) {
            Ok(_) => println!("Copied asset {:?} to {:?}", static_asset.path, target),
            Err(e) => panic!(
                "Failed to copy asset {:?} to {:?}, with error {e}",
                static_asset.path, target
            ),
        }
    }
}

pub fn generate_package(dp: &DataPackage, target: &Target, output: &Path) -> () {
    println!("Going to generate a data-package in {:?}", target);
    let generator = target.generator_for_package(dp);

    let out_root_dir = output.join(generator.root_dir());

    // PAT-3370: Output static files.
    output_static_assets(&generator, &out_root_dir)

    // TODO(PAT-3369): Generate and output table classes for each resource.
    // TODO(PAT-3370): Output dataset, entry-point files.
    // TODO(PAT-3370): Output package descriptor.
}
