use std::env;
use std::fs::{self};
use std::path::Path;
use std::path::PathBuf;

use crate::integration_test::target_tester::{exec_cmd, TargetTester};
pub struct Nodejs {}

impl TargetTester for Nodejs {
    fn build_snowflake(&self, current_dir: &PathBuf) {
        let home_dir = current_dir.as_path();

        let _nodejs_stdout = exec_cmd(
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
                "nodejs",
            ],
        );
        // assert generated directory is not empty
        assert!(
            !fs::read_dir("./tests/resources/generated/nodejs/test-snowflake@0.1.0-0.1.0")
                .map_err(|e| format!("Failed to read directory: {}", e))
                .unwrap()
                .next()
                .is_none()
        );
    }
    fn build_patch(&self, current_dir: &PathBuf) {
        let home_dir = current_dir.as_path();

        let _nodejs_stdout = exec_cmd(
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
    fn install_packages(&self, current_dir: &PathBuf) {
        let nodejs_dir = current_dir.join(Path::new("./tests/nodejs/"));
        let package_names = vec!["test-patch", "test-snowflake"];
        for name in package_names {
            let tar_path = format!(
                "../resources/generated/nodejs/{name}-0.1.0-0.1.0.tgz",
                name = name
            );
            exec_cmd(&nodejs_dir, "npm", &["install", &tar_path]);
            let ls_stdout = exec_cmd(&nodejs_dir, "npm", &["ls"]);
            assert!(ls_stdout.contains(&name));
        }
    }
    fn test_packages(&self, current_dir: &PathBuf) {
        let nodejs_dir = current_dir.join(Path::new("./tests/nodejs/"));
        // Uses env vars if present (in GH Actions, for example). Otherwise uses sops encrypted variables.
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
    fn cleanup(&self) -> std::io::Result<()> {
        fs::remove_dir_all("./tests/nodejs/node_modules")?;
        fs::remove_file("./tests/nodejs/package-lock.json")?;
        Ok(())
    }
}
