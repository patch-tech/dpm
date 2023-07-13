mod integration_test {
    pub mod python;
    pub mod target;
}

use integration_test::python;
use std::env;
use std::fs::{self};
use std::path::PathBuf;

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
    Ok(())
}

#[test]
fn integration_test() {
    if let Ok(current_dir) = env::current_dir() {
        startup().expect("failed to generate directories");
        python::build_patch(&current_dir);
        python::install_packages(&current_dir);
        python::test_packages(&current_dir);
        cleanup().expect("failed to remove generated directories");
    } else {
        eprintln!("Failed to get current directory");
    }
}
