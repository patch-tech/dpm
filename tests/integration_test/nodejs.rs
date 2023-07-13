use std::env;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

use crate::integration_test::target::exec_cmd;

pub fn build_patch(current_dir: &PathBuf) {
    let home_dir = current_dir.as_path();

    let _python_stdout = exec_cmd(
        &home_dir,
        "cargo",
        &[
            "run",
            "build-package",
            "-d",
            "./tests/resources/patch_datapackage.json",
            "-o",
            "./tests/resources/generated",
            "-y",
            "nodejs",
        ],
    );
    // assert generated directory is not empty
    assert!(
        !fs::read_dir("./tests/resources/generated/nodejs/test-patch@0.1.0-0.1.0")
            .map_err(|e| format!("Failed to read directory: {}", e))
            .unwrap()
            .next()
            .is_none()
    );
}
pub fn install_packages(current_dir: &PathBuf) {
    let nodejs_dir = current_dir.join(Path::new("./tests/nodejs/"));
    let nodejs_tar_path = "../resources/generated/nodejs/test-patch-0.1.0-0.1.0.tgz";
    exec_cmd(&nodejs_dir, "npm", &["install", nodejs_tar_path]);
    // check that package is installed
    assert!(exec_cmd(&nodejs_dir, "npm", &["ls"]).contains("test-patch"));
}
pub fn test_packages(current_dir: &PathBuf) {
    let nodejs_dir = current_dir.join(Path::new("./tests/nodejs/"));
    if env::var("PATCH_AUTH_TOKEN").is_ok() {
        exec_cmd(&nodejs_dir, "npm", &["run", "test"]);
    } else {
        exec_cmd(
            &nodejs_dir,
            "bash",
            &[
                "-e",
                "-c",
                "sops exec-env ../../secrets/dpm.enc.env 'npm run test'",
            ],
        );
    }
}
