mod generator;
mod typescript;

use std::fs;
use std::path::Path;

use super::command::Target;
use super::descriptor::DataPackage;
use generator::Generator;
use typescript::TypeScript;

fn make_generator<'a>(dp: &'a DataPackage, target: &Target) -> impl Generator + 'a {
    match target {
        Target::TypeScript => TypeScript { data_package: dp },
    }
}

fn output_static_assets(generator: &impl Generator, output: &Path) {
    for static_asset in generator.static_assets() {
        let target = output.join(&static_asset.path);
        let parent = target.parent().unwrap();
        let create_dir_res = fs::create_dir_all(parent);
        if create_dir_res.is_err() {
            panic!("Failed to create parent directories for {:?}, with error {:?}", target, create_dir_res.err());
        }

        match fs::write(&target, static_asset.asset.data) {
            Ok(_) => {
                println!(
                    "Copied asset {:?} to {:?}",
                    static_asset.path,
                    target
                );
            }
            Err(e) => {
                panic!(
                    "Failed to copy asset {:?} to {:?}, with error {e}",
                    static_asset.path,
                    target
                )
            }
        }
    }
}

pub fn generate_package(dp: &DataPackage, target: &Target, output: &Path) -> () {
    println!("Going to generate a data-package in {:?}", target);
    let generator = make_generator(dp, target);

    let out_root_dir = output.join(generator.root_dir());
    let out_src_dir = out_root_dir.join(generator.source_dir());

    // PAT-3370: Output static files.
    output_static_assets(&generator, &out_src_dir)

    // TODO(PAT-3369): Generate and output table classes for each resource.
    // TODO(PAT-3370): Output dataset, entry-point files.
    // TODO(PAT-3370): Output package descriptor.
}
