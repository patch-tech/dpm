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
use integration_test::target_tester::{
    create_snowflake_source, describe_snowflake, publish_snowflake_package, TargetTester,
};

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

/// Integration test for a given target.
///
/// Exercises the following:
///
/// 1. Target-agnostic steps: a. `dpm source create` b. `dpm describe` c. `dpm
///   publish`
/// 2. `dpm build-package`
/// 5. Install the package instance
/// 6. Run its tests
///
/// In general, the components under test are selected via the standard
/// environment variables:
/// - The API and agent under test are determined via DPM_API_URL and
///   DPM_AGENT_URL.
/// - The DPM Cloud account used follows the same lookup procedure used by `dpm`
///   and package instances: DPM_AUTH_TOKEN, else the session.json token.
fn test_target(tester: impl TargetTester) {
    let curr_dir = env::current_dir().expect("Failed to get current directory");

    startup().expect("failed to generate directories");
    let source_name = create_snowflake_source(&curr_dir);
    describe_snowflake(&curr_dir, &source_name);
    publish_snowflake_package(&curr_dir);

    tester.build_packages(&curr_dir, "test-snowflake@0.1.0");
    tester.install_packages(&curr_dir);
    tester.test_packages(&curr_dir);
}
