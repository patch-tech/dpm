mod integration_test {
    pub mod nodejs;
    pub mod python;
    pub mod target_tester;
}

use std::env;
use std::fs::{self};
use std::path::PathBuf;

use integration_test::nodejs::Nodejs;
use integration_test::python::Python;
use integration_test::target_tester::{create_snowflake_source, describe_snowflake, TargetTester};

fn startup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::create_dir_all(&path)?;
    Ok(())
}

#[test]
fn test_nodejs() {
    test_target(Nodejs {});
}

#[test]
fn test_python() {
    test_target(Python {});
}

fn test_target(_tester: impl TargetTester) {
    let curr_dir = env::current_dir().expect("Failed to get current directory");

    startup().expect("failed to generate directories");
    let source_name = create_snowflake_source(&curr_dir);
    describe_snowflake(&curr_dir, &source_name);

    // TODO(PAT-3677): Add a `dpm publish` step
    // TODO(PAT-3891): Update the TargetTester::build_packages impls in light of the
    // changes to that CLI command's syntax.
    // TODO(PAT-3937): Drop any remaining early return.
    // tester.build_packages(&curr_dir);
    // tester.install_packages(&curr_dir);
    // tester.test_packages(&curr_dir);
}
