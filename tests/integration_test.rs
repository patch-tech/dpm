mod integration_test {
    pub mod python;
    pub mod target_tester;
}

use std::env;
use std::fs::{self};
use std::path::PathBuf;

use integration_test::python::Python;
use integration_test::target_tester::TargetTester;

fn startup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::create_dir_all(&path)?;
    Ok(())
}

fn cleanup() -> std::io::Result<()> {
    let path = PathBuf::from("./tests/resources/generated/");
    fs::remove_dir_all(&path)?;
    fs::remove_dir_all("./tests/python/.venv")?;
    fs::remove_dir_all("./tests/python/__pycache__")?;
    fs::remove_dir_all("./tests/nodejs/node_modules")?;
    fs::remove_file("./tests/nodejs/package-lock.json")?;
    Ok(())
}

#[test]
fn integration_test() {
    let all_tests: Vec<Box<dyn TargetTester>> = vec![Box::new(Python {})];

    if let Ok(curr_dir) = env::current_dir() {
        startup().expect("failed to generate directories");
        for test in all_tests {
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

#[test]
fn integration_test() {
    if let Ok(current_dir) = env::current_dir() {
        startup().expect("failed to generate directories");
        for test in all_tests {
            test.build_patch(&curr_dir);
            test.install_package(&curr_dir);
            test.test_package(&curr_dir);
        }
        cleanup().expect("failed to remove generated directories");
    } else {
        eprintln!("Failed to get current directory");
    }
}
