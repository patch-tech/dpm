use convert_case::{Case, Casing};
use std::env;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

use crate::integration_test::target_tester::{exec_cmd, TargetTester};

pub struct Python {}

impl TargetTester for Python {
    fn build_snowflake(&self, current_dir: &PathBuf) {
        let home_dir = current_dir.as_path();

        let _python_stdout = exec_cmd(
            &home_dir,
            "cargo",
            &[
                "run",
                "build-package",
                "-d",
                "./tests/resources/generated/datapackage.json",
                "-o",
                "./tests/resources/generated",
                "-y",
                "python",
            ],
        );
        // assert generated directory is not empty
        assert!(
            !fs::read_dir("./tests/resources/generated/python/test-snowflake@0.1.0.0.1.0")
                .map_err(|e| format!("Failed to read directory: {}", e))
                .unwrap()
                .next()
                .is_none()
        );
    }
    fn build_patch(&self, current_dir: &PathBuf) {
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
    fn install_packages(&self, current_dir: &PathBuf) {
        let python_dir = current_dir.join(Path::new("./tests/python/"));
        let _build_venv = exec_cmd(&python_dir, "python3", &["-m", "venv", ".venv"]);
        let package_names = vec!["test-patch", "test-snowflake"];
        for name in package_names {
            let wheel_path = format!(
                "../resources/generated/python/{}@0.1.0.0.1.0/dist/{}-0.1.0.0.1.0-py3-none-any.whl",
                &name,
                &name.to_case(Case::Snake)
            );
            exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    format!("source .venv/bin/activate\npython3 -m pip install --upgrade pip\npip install pytest-asyncio\npip install {} --force-reinstall", wheel_path).as_str(),
                ],
            );
            let installation_check = exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    &format!(
                        "source .venv/bin/activate\npython3 -m pip list --local | grep {}",
                        &name
                    ),
                ],
            );
            assert!(installation_check.starts_with(&name));
            assert!(installation_check.ends_with("0.1.0.0.1.0\n"));
        }
    }
    fn test_packages(&self, current_dir: &PathBuf) {
        let python_dir = current_dir.join(Path::new("./tests/python/"));
        // Uses env vars if present (in GH Actions, for example). Otherwise uses sops encrypted variables.
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
        if env::var("SNOWSQL_ACCOUNT").is_ok()
            && env::var("SNOWSQL_USER").is_ok()
            && env::var("SNOWSQL_PWD").is_ok()
            && env::var("SNOWSQL_DATABASE").is_ok()
            && env::var("SNOWSQL_SCHEMA").is_ok()
        {
            exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    "source .venv/bin/activate\npytest -s snowflake_test.py",
                ],
            );
        } else {
            exec_cmd(
                &python_dir,
                "bash",
                &[
                    "-e",
                    "-c",
                    "source .venv/bin/activate\nsops exec-env ../../secrets/dpm.enc.env 'pytest -s snowflake_test.py'",
                ],
            );
        }
    }
    fn cleanup(&self) -> std::io::Result<()> {
        fs::remove_dir_all("./tests/python/.venv")?;
        fs::remove_dir_all("./tests/python/__pycache__")?;
        Ok(())
    }
}
