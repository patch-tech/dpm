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
use integration_test::target_tester::{describe_snowflake, TargetTester};

fn startup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::create_dir_all(&path)?;
    Ok(())
}

fn cleanup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::remove_dir_all(&path)?;
    Ok(())
}

#[test]
fn integration_test() {
    let all_tests: Vec<Box<dyn TargetTester>> = vec![Box::new(Python {}), Box::new(Nodejs {})];

    if let Ok(curr_dir) = env::current_dir() {
        startup().expect("failed to generate directories");
        describe_snowflake(&curr_dir);
        for test in all_tests {
            test.build_snowflake(&curr_dir);
            test.build_patch(&curr_dir);
            test.install_package(&curr_dir);
            test.test_package(&curr_dir);
            test.cleanup().expect("failed to remove target directories");
        }
        cleanup().expect("failed to remove generated directories");
    } else {
        eprintln!("Failed to get current directory");
    }
}
