mod generator;
mod typescript;

use super::command::Target;
use super::descriptor::DataPackage;
use generator::Generator;
use typescript::TypeScript;

pub fn generate_package(_dp: &DataPackage, target: &Target) -> () {
    println!("Going to generate a data-package in {:?}", target);
    let _generator: &dyn Generator = match target {
        Target::TypeScript => &TypeScript,
    };

    // TODO(PAT-3370): Output static files.
    // TODO(PAT-3369): Generate and output table classes for each resource.
    // TODO(PAT-3370): Output dataset, entry-point files.
    // TODO(PAT-3370): Output package descriptor.
}
