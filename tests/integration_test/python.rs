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
            "python",
        ],
    );
    // assert generated directory is not empty
    assert!(
        !fs::read_dir("./tests/resources/generated/python/test-patch@0.1.0.0.1.0")
            .map_err(|e| format!("Failed to read directory: {}", e))
            .unwrap()
            .next()
            .is_none()
    );
}
pub fn install_packages(current_dir: &PathBuf) {
    let python_dir = current_dir.join(Path::new("./tests/python/"));
    let package_wheel_path =
                    "../resources/generated/python/test-patch@0.1.0.0.1.0/dist/test_patch-0.1.0.0.1.0-py3-none-any.whl";
    let _build_venv = exec_cmd(&python_dir, "python3", &["-m", "venv", ".venv"]);
    exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    format!("source .venv/bin/activate\npython3 -m pip install --upgrade pip\npip install pytest-asyncio\npip install {} --force-reinstall", package_wheel_path).as_str(),
                ],
            );
    // check that package is installed
    let python_package_check = exec_cmd(
        &python_dir,
        "bash",
        &[
            "-e",
            "-c",
            "source .venv/bin/activate\npython3 -m pip list --local | grep test-patch",
        ],
    );
    assert!(python_package_check.starts_with("test-patch"));
    assert!(python_package_check.ends_with("0.1.0.0.1.0\n"));
}
pub fn test_packages(current_dir: &PathBuf) {
    let python_dir = current_dir.join(Path::new("./tests/python/"));
    if env::var("PATCH_AUTH_TOKEN").is_ok() {
        exec_cmd(
            &python_dir,
            "bash",
            &[
                "-e",
                "-c",
                "source .venv/bin/activate\npytest -s patch_test.py",
            ],
        );
    } else {
        exec_cmd(
            &python_dir,
            "bash",
            &[
                "-e",
                "-c",
                "source .venv/bin/activate\nsops exec-env ../../secrets/dpm.enc.env 'pytest -s patch_test.py'",
            ],
        );
    }
}
